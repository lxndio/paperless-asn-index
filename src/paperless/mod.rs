pub mod correspondents;
pub mod documents;

use std::collections::HashMap;

use actix_web::{get, post, web, App, HttpServer, Result};
use maud::{html, Markup};
use reqwest::Client;
use serde::Deserialize;

use crate::paperless::{correspondents::Correspondents, documents::Documents};

use self::documents::Document;

pub enum PaperlessType {
    Correspondents,
    Documents,
}

pub async fn get_correspondents(
    paperless_url: &str,
    paperless_token: &str,
    client: &Client,
) -> Result<HashMap<u64, String>> {
    let mut next = Some(format!("{}/correspondents/", paperless_url));
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
            .header("Authorization", format!("Token {}", paperless_token))
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

    Ok(correspondents_map)
}

pub async fn get_documents(
    paperless_url: &str,
    paperless_token: &str,
    asn_from: &str,
    asn_to: &str,
    client: &Client,
) -> Result<Vec<Document>> {
    let mut next = Some(format!(
        "{}/documents/?query=asn:[{} TO {}]",
        paperless_url, asn_from, asn_to
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
            .header("Authorization", format!("Token {}", paperless_token))
            .send()
            .await
            .expect("Failed to send request");

        let mut content = res.json::<Documents>().await.expect("Failed to parse JSON");

        next = content.next;
        documents.append(&mut content.results);
    }

    Ok(documents)
}
