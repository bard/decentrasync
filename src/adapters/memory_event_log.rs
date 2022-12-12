use std::sync::Mutex;

use crate::app::{BookmarkCreatedEvent, EventLog};

pub struct MemoryEventLog {
    events: Mutex<Vec<BookmarkCreatedEvent>>,
}

impl MemoryEventLog {
    pub fn new() -> Self {
        let events: Mutex<Vec<BookmarkCreatedEvent>> = Mutex::new(vec![]);
        Self { events }
    }
}

impl EventLog for MemoryEventLog {
    fn push(&self, event: BookmarkCreatedEvent) -> Result<(), ()> {
        let mut lock = match self.events.lock() {
            Ok(lock) => lock,
            _ => panic!(), //return Err(InsertError::Unknown),
        };

        lock.push(event.clone());
        Ok(())
    }
}
