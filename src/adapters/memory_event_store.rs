use std::sync::Mutex;

use crate::app::{DomainEvent, EventStore};

pub struct MemoryEventStore {
    events: Mutex<Vec<DomainEvent>>,
}

impl MemoryEventStore {
    pub fn new() -> Self {
        let events: Mutex<Vec<DomainEvent>> = Mutex::new(vec![]);
        Self { events }
    }
}

impl EventStore for MemoryEventStore {
    fn push(&self, event: DomainEvent) -> Result<(), ()> {
        let mut lock = match self.events.lock() {
            Ok(lock) => lock,
            _ => panic!(),
        };

        lock.push(event.clone());
        Ok(())
    }

    fn get_all_events(&self) -> Vec<DomainEvent> {
        match self.events.lock() {
            Ok(lock) => lock.to_vec(), // XXX how to avoid copying vector?
            _ => panic!(),
        }
    }
}
