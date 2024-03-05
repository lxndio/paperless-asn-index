use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Correspondents {
    pub count: u64,
    pub next: Option<u64>,
    pub previous: Option<u64>,
    pub all: Vec<u64>,
    pub results: Vec<Correspondent>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Correspondent {
    pub id: u64,
    pub slug: String,
    pub name: String,
    // match: String,
    // matching_algorithm: u64,
    // is_insensitive: bool,
    // document_count: u64,
    // last_correspondence: String,
    // owner: u64,
    // user_can_change: bool,
}
