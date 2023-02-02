use serde::Serialize;
use std::time::SystemTime;

pub type BookmarkId = String;

#[derive(std::fmt::Debug, PartialEq, Eq, Clone, Serialize)]
pub struct DomainEvent {
    pub meta: DomainEventMeta,
    pub payload: DomainEventPayload,
}

#[derive(std::fmt::Debug, PartialEq, Eq, Clone, Serialize)]
pub struct DomainEventMeta {
    pub created_at: SystemTime,
}

#[derive(std::fmt::Debug, PartialEq, Eq, Clone, Serialize)]
pub enum DomainEventPayload {
    BookmarkCreated {
        id: BookmarkId,
        url: String,
        title: String,
    },
    BookmarkDeleted {
        id: BookmarkId,
    },
    BookmarkTitleUpdated {
        id: BookmarkId,
        title: String,
    },
}

#[derive(std::fmt::Debug, PartialEq, Eq, Clone)]
pub struct Bookmark {
    pub id: BookmarkId,
    pub url: String,
    pub title: String,
}

#[derive(std::fmt::Debug)]
pub struct BookmarkInput {
    pub url: String,
    pub title: String,
}

#[derive(std::fmt::Debug)]
pub struct BookmarkQuery {
    pub id: BookmarkId,
}
