use std::sync::Mutex;

use crate::app::{DomainEvent, EventLog};

pub struct MemoryEventLog {
    events: Mutex<Vec<DomainEvent>>,
}

impl MemoryEventLog {
    pub fn new() -> Self {
        let events: Mutex<Vec<DomainEvent>> = Mutex::new(vec![]);
        Self { events }
    }
}

impl EventLog for MemoryEventLog {
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
