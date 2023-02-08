use crate::{
    domain::data::DomainEvent,
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

    fn get_events_for_aggregate(&self, aggregate_id: &str) -> Vec<DomainEvent> {
        self.events
            .lock()
            .unwrap()
            .iter()
            .filter(|e| e.meta.aggregate_id == aggregate_id)
            .map(|e| e.clone())
            .collect()
    }

    fn events_iter(&self) -> Box<dyn Iterator<Item = DomainEvent>> {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;
    use crate::domain::events::{BookmarkEventPayload, DomainEventPayload};
    use crate::ports::Clock;
    use crate::{adapters::clock::FakeClock, domain::data::DomainEventMeta};

    #[test]
    fn test_importing_an_event_sorts_the_log() {
        let event_store = MemoryEventStore::new();
        let clock = FakeClock::new();

        let earlier_external_event_time = clock.now();
        let earlier_external_event = DomainEvent {
            meta: DomainEventMeta {
                aggregate_id: "abc".to_owned(),
                created_at: earlier_external_event_time,
            },
            payload: DomainEventPayload::Bookmark(BookmarkEventPayload::Created {
                url: "https://google.com".to_owned(),
                title: "Google".to_owned(),
            }),
        };

        clock.advance(Duration::from_secs(10));
        let later_local_event_time = clock.now();
        let later_local_event = DomainEvent {
            meta: DomainEventMeta {
                aggregate_id: "123".to_owned(),
                created_at: later_local_event_time,
            },
            payload: DomainEventPayload::Bookmark(BookmarkEventPayload::Created {
                url: "https://example.com".to_owned(),
                title: "Example".to_owned(),
            }),
        };

        event_store.store_event(later_local_event).unwrap();
        event_store.import_event(earlier_external_event).unwrap();

        let events = event_store.events.lock().unwrap();

        assert_eq!(events[0].meta.created_at, earlier_external_event_time);
    }
}
