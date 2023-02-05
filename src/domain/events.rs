use serde::{Deserialize, Serialize};

#[derive(std::fmt::Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum DomainEventPayload {
    Bookmark(BookmarkEventPayload),
    Other(OtherEventPayload),
}

#[derive(std::fmt::Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[serde(tag = "event", rename_all = "snake_case")]
pub enum BookmarkEventPayload {
    Created { url: String, title: String },
    Deleted,
    TitleUpdated { title: String },
}

#[derive(std::fmt::Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum OtherEventPayload {}
