use super::{errors::DomainError, events::DomainEventPayload};
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

#[derive(std::fmt::Debug, PartialEq, Eq, Clone)]
pub struct BookmarkData {
    pub id: String,
    pub url: String,
    pub title: String,
}

#[derive(std::fmt::Debug)]
pub struct BookmarkQuery {
    pub id: String,
}

#[derive(std::fmt::Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct DomainEvent {
    pub meta: DomainEventMeta,
    pub payload: DomainEventPayload,
}

#[derive(std::fmt::Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct DomainEventMeta {
    pub aggregate_id: String,
    pub created_at: SystemTime,
}

pub trait Aggregate {
    type Command;
    type EventPayload;

    fn apply_event(self, event: &Self::EventPayload, meta: &DomainEventMeta) -> Self;
    fn handle_command(&self, command: &Self::Command) -> Result<Self::EventPayload, DomainError>;
}
