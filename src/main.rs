mod correspondents;
mod documents;

use std::collections::HashMap;

use actix_web::{get, post, web, App, HttpServer, Result};
use maud::{html, Markup};
use serde::Deserialize;

use crate::{correspondents::Correspondents, documents::Documents};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            // .wrap(Logger::default())
            .service(actix_files::Files::new("/static", "./static"))
            .service(site)
            .service(show_index)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

#[get("/")]
async fn site() -> Result<Markup> {
    Ok(html! {
        html {
            head {
                title { "Paperless ASN Index Generator" }
                link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/bulma@0.9.4/css/bulma.min.css";
                script src="https://code.jquery.com/jquery-3.7.1.min.js" {}
            }
            body {
                section class="section" {
                    div class="container" {
                        h1 class="title" { "Paperless ASN Index Generator" }

                        form action="" id="form" {
                            div class="field" {
                                label class="label" { "Paperless API URL" }
                                div class="control" {
                                    input class="input" type="text" id="paperless_url" placeholder="https://paperless.example.com/api";
                                }
                            }

                            div class="field" {
                                label class="label" { "Paperless API Token" }
                                div class="control" {
                                    input class="input" type="text" id="paperless_token" placeholder="your-api-token";
                                }
                            }

                            div class="field" {
                                label class="label" { "ASN Range" }
                                div class="field has-addons" {
                                    div class="control" {
                                        input class="input" type="text" id="asn_from" placeholder="From";
                                    }
                                    div class="control" {
                                        a class="button is-static" { "–" }
                                    }
                                    div class="control" {
                                        input class="input" type="text" id="asn_to" placeholder="To";
                                    }
                                }
                            }

                            div class="columns" {
                                div class="column" {
                                    div class="field" {
                                        label class="label" { "Show fields" }
                                        div class="select is-multiple" {
                                            select multiple size="6" id="show_fields" {
                                                option value="ID" { "ID" }
                                                option value="ASN" { "ASN" }
                                                option value="Correspondent" { "Correspondent" }
                                                option value="Title" { "Title" }
                                                option value="Tags" { "Tags" }
                                                option value="Created Date" { "Created Date" }
                                            }
                                        }
                                    }
                                }

                                div class="column" {
                                    div class="field" {
                                        label class="label" { "Sort by" }
                                        div class="select" {
                                            select id="sort_by" {
                                                option value="ID" { "ID" }
                                                option value="ASN" { "ASN" }
                                                option value="Correspondent" { "Correspondent" }
                                                option value="Title" { "Title" }
                                                option value="Created Date" { "Created Date" }
                                            }
                                        }
                                    }
                                }
                            }

                            div class="field" {
                                div class="control" {
                                    button class="button is-link" type="submit" { "Generate Index" }
                                }
                            }
                        }
                    }
                }

                script type="text/javascript" src="/static/submit_form.js" {}
            }
        }
    })
}

#[derive(Debug, Deserialize)]
struct ShowIndexFormData {
    pub paperless_url: String,
    pub paperless_token: String,
    pub asn_from: String,
    pub asn_to: String,
    pub show_fields: Vec<String>,
    pub sort_by: String,
}

#[post("/show_index")]
async fn show_index(form: web::Json<ShowIndexFormData>) -> Result<Markup> {
    let client = reqwest::Client::new();

    // Get correspondents
    let mut next = Some(format!("{}/correspondents/", form.paperless_url));
    let mut correspondents = Vec::new();

    while let Some(ref url) = next {
        // Make URL https if it's not
        let url = if url.starts_with("http://") {
            url.replacen("http://", "https://", 1)
        } else {
            url.to_string()
        };

        let res = client
            .get(url)
            .header("Authorization", format!("Token {}", form.paperless_token))
            .send()
            .await
            .expect("Failed to send request");

        let mut content = res
            .json::<Correspondents>()
            .await
            .expect("Failed to parse JSON");

        next = content.next;
        correspondents.append(&mut content.results);
    }

    let mut correspondents_map = HashMap::new();
    for correspondent in correspondents {
        correspondents_map.insert(correspondent.id, correspondent.name);
    }

    // Get documents
    let mut next = Some(format!(
        "{}/documents/?query=asn:[{} TO {}]",
        form.paperless_url, form.asn_from, form.asn_to
    ));
    let mut documents = Vec::new();

    while let Some(ref url) = next {
        // Make URL https if it's not
        let url = if url.starts_with("http://") {
            url.replacen("http://", "https://", 1)
        } else {
            url.to_string()
        };

        let res = client
            .get(url)
            .header("Authorization", format!("Token {}", form.paperless_token))
            .send()
            .await
            .expect("Failed to send request");

        let mut content = res.json::<Documents>().await.expect("Failed to parse JSON");

        next = content.next;
        documents.append(&mut content.results);
    }

    // Sort documents
    documents.sort_by(|a, b| match form.sort_by.as_str() {
        "ID" => a.id.cmp(&b.id),
        "ASN" => a.archive_serial_number.cmp(&b.archive_serial_number),
        "Correspondent" => a.correspondent.cmp(&b.correspondent),
        "Title" => a.title.cmp(&b.title),
        "Created Date" => a.created_date.cmp(&b.created_date),
        _ => a.id.cmp(&b.id),
    });

    // Render table
    Ok(html! {
        head {
            title { "Paperless ASN Index" }
            link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/bulma@0.9.4/css/bulma.min.css";
        }
        body {
            section class="section" {
                table class="table is-size-7 is-narrow" {
                    thead {
                        tr {
                            @for field in &form.show_fields {
                                th { (field) }
                            }
                        }
                    }
                    tbody {
                        @for document in &documents {
                            tr {
                                @for field in &form.show_fields {
                                    @match field.as_str() {
                                        "ID" => td { (document.id) },
                                        "ASN" => td { (document.archive_serial_number) },
                                        "Correspondent" => td { (correspondents_map[&document.correspondent]) },
                                        "Title" => td { (document.title) },
                                        "Tags" => {
                                            td {
                                                @for tag in &document.tags {
                                                    (tag) ", "
                                                }
                                            }
                                        }
                                        "Created Date" => td { (document.created_date) },
                                        _ => ""
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    })
}
