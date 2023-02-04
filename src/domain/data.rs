use super::{errors::DomainError, events::DomainEventPayload};
use serde::Serialize;
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

#[derive(std::fmt::Debug, PartialEq, Eq, Clone, Serialize)]
pub struct DomainEvent {
    pub meta: DomainEventMeta,
    pub payload: DomainEventPayload,
}

#[derive(std::fmt::Debug, PartialEq, Eq, Clone, Serialize)]
pub struct DomainEventMeta {
    pub created_at: SystemTime,
}

pub trait Aggregate {
    type Command;

    fn apply_event(&mut self, event: &DomainEvent);
    fn handle_command(&self, command: &Self::Command) -> Result<DomainEventPayload, DomainError>;
}
