use crate::domain::data::{BookmarkData, DomainEvent};
use std::time::SystemTime;

pub trait EventStore: Send + Sync {
    fn store_event(&self, event: DomainEvent) -> Result<(), EventStoreError>;
    fn import_event(&self, event: DomainEvent) -> Result<(), EventStoreError>;
    fn get_events_for_aggregate(&self, aggregate_id: String) -> Vec<DomainEvent>;
}

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum EventStoreError {
    #[error("Generic event store error")]
    Generic,
}

pub trait ReadModel: Send + Sync {
    fn update(&self, event: &DomainEvent) -> Result<(), ReadModelError>;
    fn read_bookmark(&self, id: &str) -> Option<BookmarkData>;
    fn read_bookmarks(&self) -> Option<Vec<BookmarkData>>;
}

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum ReadModelError {
    #[error("Generic read model error")]
    Generic,
}

pub trait Clock: Send + Sync {
    fn now(&self) -> SystemTime;
}
