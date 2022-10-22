use serde::{self, Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MediaTitle {
    pub romaji: Option<String>,
    pub english: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MediaType {
    Anime,
    Manga,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MediaFormat {
    Tv,
    TvShort,
    Movie,
    Special,
    Ova,
    Ona,
    Music,
    Manga,
    Novel,
    OneShot,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MediaStatus {
    Finished,
    Releasing,
    NotYetReleased,
    Cancelled,
    Hiatus,
}

impl MediaStatus {
    pub fn str(&self) -> &'static str {
        match self {
            MediaStatus::Finished => "Finished",
            MediaStatus::Releasing => "Releasing",
            MediaStatus::NotYetReleased => "Upcoming",
            MediaStatus::Cancelled => "Cancelled",
            MediaStatus::Hiatus => "Hiatus",
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MediaCoverImage {
    pub medium: Option<String>,
    pub color: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Media {
    pub id: i32,
    pub title: Option<MediaTitle>,
    #[serde(rename = "type")]
    pub media_type: Option<MediaType>,
    pub format: Option<MediaFormat>,
    pub description: Option<String>,
    pub status: Option<MediaStatus>,
    pub episodes: Option<i32>,
    pub chapters: Option<i32>,
    pub cover_image: Option<MediaCoverImage>,
    pub banner_image: Option<String>,
    pub average_score: Option<i32>,
}
