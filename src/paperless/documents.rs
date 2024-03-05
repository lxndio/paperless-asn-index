use std::collections::HashMap;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Documents {
    pub count: u64,
    pub next: Option<String>,
    pub previous: Option<String>,
    pub all: Vec<u64>,
    pub results: Vec<Document>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Document {
    pub id: u64,
    pub correspondent: u64,
    pub document_type: u64,
    pub storage_path: u64,
    pub title: String,
    pub content: String,
    pub tags: Vec<u64>,
    pub created: String,
    pub created_date: String,
    pub modified: String,
    pub added: String,
    pub archive_serial_number: u64,
    // original_file_name: String,
    // archived_file_name: String,
    // owner: u64,
    // user_can_change: bool,
    // is_shared_by_requester: bool,
    // notes: Vec<String>,
    // custom_fields: Vec<String>,
    // __search_hit__: SearchHit,
}

#[derive(Debug, Deserialize)]
pub struct SearchHit {
    pub score: f64,
    pub highlights: Vec<String>,
    pub note_highlights: Vec<String>,
    pub rank: u64,
}

pub fn group_documents(
    documents: Vec<Document>,
    correspondents: &HashMap<u64, String>,
    group_by: &str,
    sort_by: &str,
    sort_desc: bool,
) -> HashMap<String, Vec<Document>> {
    let mut grouped_documents = HashMap::new();

    // Group documents
    for document in documents.iter() {
        let key = match group_by {
            "ID" => document.id.to_string(),
            "ASN" => document.archive_serial_number.to_string(),
            "Correspondent" => correspondents[&document.correspondent].clone(),
            "Title" => document.title.clone(),
            "Created Date" => document.created_date.clone(),
            _ => document.id.to_string(),
        };

        grouped_documents
            .entry(key)
            .or_insert_with(Vec::new)
            .push(document.clone());
    }

    // Sort documents in groups
    for (_, documents) in grouped_documents.iter_mut() {
        documents.sort_by(|a, b| match sort_by {
            "ID" => a.id.cmp(&b.id),
            "ASN" => a.archive_serial_number.cmp(&b.archive_serial_number),
            "Correspondent" => {
                correspondents[&a.correspondent].cmp(&correspondents[&b.correspondent])
            }
            "Title" => a.title.cmp(&b.title),
            "Created Date" => a.created_date.cmp(&b.created_date),
            _ => a.id.cmp(&b.id),
        });

        if sort_desc {
            documents.reverse();
        }
    }

    grouped_documents
}
