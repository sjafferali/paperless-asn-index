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

pub fn group_documents(
    documents: Vec<Document>,
    correspondents: &HashMap<u64, String>,
    group_by: &str,
    sort_by: &str,
    sort_desc: bool,
) -> HashMap<String, Vec<Document>> {
    let mut grouped_documents = HashMap::new();
    let unknown_correspondent = "Unknown Correspondent".to_string();

    // Group documents
    for document in documents.iter() {
        let key = match group_by {
            "ID" => document.id.to_string(),
            "ASN" => document.archive_serial_number.to_string(),
            "Correspondent" => {
                document.correspondent
                    .and_then(|id| correspondents.get(&id).cloned())
                    .unwrap_or_else(|| unknown_correspondent.clone())
            }
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
    }

    grouped_documents
}
