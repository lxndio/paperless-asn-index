use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Documents {
    pub count: u64,
    pub next: Option<String>,
    pub previous: Option<String>,
    pub all: Vec<u64>,
    pub results: Vec<Document>,
}

#[derive(Debug, Deserialize)]
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
