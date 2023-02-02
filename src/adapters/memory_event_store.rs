use crate::{
    data::DomainEvent,
    ports::{EventStore, EventStoreError},
};
use std::sync::Mutex;

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
    fn import_event(&self, event: DomainEvent) -> Result<(), EventStoreError> {
        let mut lock = self.events.lock().unwrap();
        lock.push(event);
        lock.sort_by(|a, b| a.meta.created_at.cmp(&b.meta.created_at));
        Ok(())
    }

    fn store_event(&self, event: DomainEvent) -> Result<(), EventStoreError> {
        let mut lock = self.events.lock().unwrap();
        lock.push(event);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;
    use crate::ports::Clock;
    use crate::{
        adapters::clock::FakeClock,
        data::{DomainEventMeta, DomainEventPayload},
    };

    #[test]
    fn test_importing_an_event_sorts_the_log() {
        let event_store = MemoryEventStore::new();
        let clock = FakeClock::new();

        let earlier_external_event_time = clock.now();
        let earlier_external_event = DomainEvent {
            meta: DomainEventMeta {
                created_at: earlier_external_event_time,
            },
            payload: DomainEventPayload::BookmarkCreated {
                id: String::from("abc"),
                url: String::from("https://google.com"),
                title: String::from("Google"),
            },
        };

        clock.advance(Duration::from_secs(10));
        let later_local_event_time = clock.now();
        let later_local_event = DomainEvent {
            meta: DomainEventMeta {
                created_at: later_local_event_time,
            },
            payload: DomainEventPayload::BookmarkCreated {
                id: String::from("123"),
                url: String::from("https://example.com"),
                title: String::from("Example"),
            },
        };

        event_store.store_event(later_local_event).unwrap();
        event_store.import_event(earlier_external_event).unwrap();

        let events = event_store.events.lock().unwrap();

        assert_eq!(events[0].meta.created_at, earlier_external_event_time);
    }
}
