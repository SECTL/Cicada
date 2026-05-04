use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AnnouncementType {
    #[serde(rename = "normal")]
    Normal,
    #[serde(rename = "emergency")]
    Emergency,
    #[serde(rename = "notice")]
    Notice,
}

impl std::fmt::Display for AnnouncementType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnnouncementType::Normal => write!(f, "normal"),
            AnnouncementType::Emergency => write!(f, "emergency"),
            AnnouncementType::Notice => write!(f, "notice"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Announcement {
    pub id: Uuid,
    pub title: String,
    pub content_html: String,
    pub announcement_type: AnnouncementType,
    pub publisher_name: String,
    pub publisher_user_id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnnouncementCreate {
    pub title: String,
    pub content_html: String,
    pub announcement_type: AnnouncementType,
    pub publisher_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnnouncementUpdate {
    pub title: Option<String>,
    pub content_html: Option<String>,
    pub announcement_type: Option<AnnouncementType>,
    pub publisher_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnnouncementListResponse {
    pub announcements: Vec<Announcement>,
    pub total: u64,
    pub page: u32,
    pub per_page: u32,
}
