use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Documents {
    pub count: u64,
    pub next: Option<String>,
    pub previous: Option<String>,
    pub all: Vec<u64>,
    pub results: Vec<Document>,
}

// src/paperless/documents.rs
#[derive(Debug, Clone, Deserialize)]
pub struct Document {
    pub id: u64,
    pub correspondent: Option<u64>, // Change to Option<u64>
    pub document_type: u64,
    pub storage_path: Option<u64>,
    pub title: String,
    pub content: String,
    pub tags: Vec<u64>,
    pub created: String,
    pub created_date: String,
    pub modified: String,
    pub added: String,
    pub archive_serial_number: u64,
}

#[derive(Debug, Deserialize)]
pub struct SearchHit {
    pub score: f64,
    pub highlights: Vec<String>,
    pub note_highlights: Vec<String>,
    pub rank: u64,
}

pub fn sort_documents(
    mut documents: Vec<Document>,
    correspondents: &std::collections::HashMap<u64, String>,
    sort_by: &str,
    sort_desc: bool,
) -> Vec<Document> {
    let unknown_correspondent = "Unknown Correspondent".to_string();

    documents.sort_by(|a, b| match sort_by {
        "ID" => a.id.cmp(&b.id),
        "ASN" => a.archive_serial_number.cmp(&b.archive_serial_number),
        "Correspondent" => {
            let a_corr = a.correspondent.and_then(|id| correspondents.get(&id)).unwrap_or(&unknown_correspondent);
            let b_corr = b.correspondent.and_then(|id| correspondents.get(&id)).unwrap_or(&unknown_correspondent);
            a_corr.cmp(b_corr)
        }
        "Title" => a.title.cmp(&b.title),
        "Created Date" => a.created_date.cmp(&b.created_date),
        _ => a.id.cmp(&b.id),
    });

    if sort_desc {
        documents.reverse();
    }

    documents
}
