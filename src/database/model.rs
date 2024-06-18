use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::{Formatter, Result as FmtResult};

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, sqlx::FromRow, Serialize, Clone)]
pub struct User {
    pub id: uuid::Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
    pub admin: bool,
    #[serde(rename = "createdAt")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<DateTime<Utc>>,
}
#[derive(Debug, Deserialize, sqlx::FromRow, Serialize, Clone)]
pub struct Post {
    pub id: uuid::Uuid,
    pub title: String,
    pub subtitle: Option<String>,
    pub category: Option<String>,
    pub content: String,
    #[serde(rename = "createdAt")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<DateTime<Utc>>,
}

pub struct Attachment {
    pub filename: String,
    pub bytes: Vec<u8>,
}

impl std::fmt::Debug for Attachment {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        writeln!(
            f,
            "{}",
            format!("{} - size {} bytes", self.filename, self.bytes.len())
        )
    }
}

#[derive(Debug, Default)]
pub struct UploadBlogPost {
    pub title: String,
    pub subtitle: String,
    pub category: Option<String>,
    pub attachments: Vec<Attachment>,
    pub content: String,
}

#[derive(Debug)]
pub struct UpdateBlogPost {
    pub title: Option<String>,
    pub subtitle: Option<String>,
    pub category: Option<String>,
    pub attachments: Vec<Attachment>,
    pub content: Option<String>,
}

impl Default for UpdateBlogPost {
    fn default() -> Self {
        Self {
            title: None,
            subtitle: None,
            attachments: vec![],
            category: None,
            content: None,
        }
    }
}

impl Attachment {
    pub fn new(filename: &str, bytes: &[u8]) -> Self {
        Attachment {
            filename: filename.to_string(),
            bytes: bytes.to_vec(),
        }
    }
}
