use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, Debug)]
enum FileType {
    Acsm,
    Book,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FileMetadata {
    pub path: String,
    pub file_type: FileType,
    pub last_attempt: Option<DateTime<Utc>>,
    pub upload_success: Option<bool>,
}

pub struct MetadataManager {
    pub metadata: HashMap<String, FileMetadata>,
}

impl MetadataManager {
    pub fn new() -> Self {
        Self { metadata: HashMap::new() }
    }
}

