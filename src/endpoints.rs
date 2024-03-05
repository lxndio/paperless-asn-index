use std::collections::HashMap;

use actix_web::{get, post, web, App, HttpServer, Result};
use maud::{html, Markup};
use serde::Deserialize;

use crate::paperless::{
    documents::{self, group_documents},
    get_correspondents, get_documents,
};

#[derive(Debug, Deserialize)]
struct ShowIndexFormData {
    pub paperless_url: String,
    pub paperless_token: String,
    pub asn_from: String,
    pub asn_to: String,
    pub show_fields: Vec<String>,
    pub group_by: String,
    pub sort_by: String,
    pub sort_desc: bool,
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
                                        label class="label" { "Group by" }
                                        div class="select" {
                                            select id="group_by" {
                                                option value="ID" { "ID" }
                                                option value="ASN" { "ASN" }
                                                option value="Correspondent" { "Correspondent" }
                                                option value="Title" { "Title" }
                                                option value="Created Date" { "Created Date" }
                                            }
                                        }
                                    }

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

                                    label class="checkbox" {
                                        input type="checkbox" id="sort_desc" {}
                                            " Descending"
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

#[post("/show_index")]
async fn show_index(form: web::Json<ShowIndexFormData>) -> Result<Markup> {
    let client = reqwest::Client::new();

    let correspondents_map =
        get_correspondents(&form.paperless_url, &form.paperless_token, &client).await?;
    let documents = get_documents(
        &form.paperless_url,
        &form.paperless_token,
        &form.asn_from,
        &form.asn_to,
        &client,
    )
    .await?;
    let grouped_documents = group_documents(
        documents,
        &correspondents_map,
        &form.group_by,
        &form.sort_by,
        form.sort_desc,
    );

    // Get sorted list of groups
    let mut groups: Vec<&String> = grouped_documents.keys().collect();
    groups.sort();

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
                        @for group in groups {
                            tr {
                                th colspan=(form.show_fields.len()) { (group) }
                            }
                            @for document in grouped_documents[group].iter() {
                                tr {
                                    @for field in &form.show_fields {
                                        td {
                                            @match field.as_str() {
                                                "ID" => { (document.id) }
                                                "ASN" => { (document.archive_serial_number) }
                                                "Correspondent" => { (correspondents_map[&document.correspondent]) }
                                                "Title" => { (document.title) }
                                                "Tags" => { (document.tags.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(", ")) }
                                                "Created Date" => { (document.created_date) }
                                                _ => { "" }
                                            }
                                        }
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
