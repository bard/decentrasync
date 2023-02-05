use serde::Serialize;

#[derive(std::fmt::Debug, PartialEq, Eq, Clone, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum DomainEventPayload {
    Bookmark(BookmarkEventPayload),
    Other(OtherEventPayload),
}

#[derive(std::fmt::Debug, PartialEq, Eq, Clone, Serialize)]
#[serde(tag = "event", rename_all = "snake_case")]
pub enum BookmarkEventPayload {
    Created { url: String, title: String },
    Deleted,
    TitleUpdated { title: String },
}

#[derive(std::fmt::Debug, PartialEq, Eq, Clone, Serialize)]
pub enum OtherEventPayload {}
