use crate::app::{DomainEvent, EventStore};
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
    fn import_event(&self, event: DomainEvent) {
        match self.events.lock() {
            Ok(mut lock) => {
                lock.push(event);
                lock.sort_by(|a, b| a.meta.created_at.cmp(&b.meta.created_at));
            }
            _ => panic!(),
        }
    }

    fn store_event(&self, event: DomainEvent) {
        match self.events.lock() {
            Ok(mut lock) => lock.push(event),
            _ => panic!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use crate::app::{DomainEventMeta, DomainEventPayload};

    use super::*;
    use mock_instant::{Instant, MockClock};

    #[test]
    fn test_importing_an_event_sorts_the_log() {
        let event_store = MemoryEventStore::new();

        let earlier_external_event_time = Instant::now();
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

        MockClock::advance(Duration::from_secs(10));

        event_store.store_event(DomainEvent {
            meta: DomainEventMeta {
                created_at: Instant::now(),
            },
            payload: DomainEventPayload::BookmarkCreated {
                id: String::from("123"),
                url: String::from("https://example.com"),
                title: String::from("Example"),
            },
        });

        event_store.import_event(earlier_external_event);

        let events = event_store.events.lock().unwrap();

        assert_eq!(events[0].meta.created_at, earlier_external_event_time);
    }
}
