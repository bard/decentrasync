use crate::app::{Bookmark, BookmarkQuery, DomainEvent};
use std::time::SystemTime;

pub trait EventStore: Send + Sync {
    fn store_event(&self, event: DomainEvent) -> Result<(), EventStoreError>;
    fn import_event(&self, event: DomainEvent) -> Result<(), EventStoreError>;
}

#[derive(thiserror::Error, Debug)]
pub enum EventStoreError {
    #[error("Generic event store error")]
    Generic,
}

pub trait ReadModel: Send + Sync {
    fn update(&self, event: &DomainEvent) -> Result<(), ReadModelError>;
    fn read_bookmark(&self, query: &BookmarkQuery) -> Option<Bookmark>;
    fn read_bookmarks(&self) -> Option<Vec<Bookmark>>;
}

#[derive(thiserror::Error, Debug)]
pub enum ReadModelError {
    #[error("Generic read model error")]
    Generic,
}

pub trait Clock: Send + Sync {
    fn now(&self) -> SystemTime;
}
