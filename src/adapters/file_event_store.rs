use crate::{domain::data::DomainEvent, ports::EventStore, ports::EventStoreError};
use std::{
    ffi::{OsStr, OsString},
    fs,
    path::Path,
    time::UNIX_EPOCH,
};

pub struct FileSystemEventStore {
    log_folder_path: OsString,
}

impl FileSystemEventStore {
    pub fn new(path: &OsStr) -> Self {
        Self {
            log_folder_path: path.to_owned(),
        }
    }
}

impl EventStore for FileSystemEventStore {
    fn import_event(&self, _event: DomainEvent) -> Result<(), EventStoreError> {
        todo!()
    }

    fn store_event(&self, event: DomainEvent) -> Result<(), EventStoreError> {
        let timestamp_millis = event
            .meta
            .created_at
            .duration_since(UNIX_EPOCH)
            .map_err(|_source| EventStoreError::Generic)?
            .as_millis();

        let stored_event_path =
            Path::new(self.log_folder_path.as_os_str()).join(format!("{}.json", timestamp_millis));

        std::fs::write(
            stored_event_path,
            serde_json::to_string_pretty(&event).unwrap(),
        )
        .unwrap();

        Ok(())
    }

    fn get_events_for_aggregate(&self, aggregate_id: String) -> Vec<DomainEvent> {
        let mut entries = std::fs::read_dir(&self.log_folder_path)
            .unwrap()
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, std::io::Error>>()
            .unwrap();

        entries.sort();

        entries
            .iter()
            .map(|path| {
                serde_json::from_str::<DomainEvent>(&fs::read_to_string(path).unwrap()).unwrap()
            })
            .filter(|e| e.meta.aggregate_id == aggregate_id)
            .collect::<Vec<DomainEvent>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adapters::clock::FakeClock;
    use crate::domain::data::DomainEventMeta;
    use crate::domain::events::{BookmarkEventPayload, DomainEventPayload};
    use crate::ports::Clock;
    use assert_fs::assert::PathAssert;
    use assert_fs::fixture::PathChild;
    use assert_fs::TempDir;
    use std::sync::Arc;
    use std::time::Duration;

    #[test]
    fn test_storing_an_event_writes_to_disk() {
        let temp = TempDir::new().unwrap();
        let log_folder_path = temp.path().as_os_str();
        let clock = Arc::new(FakeClock::new());
        let event_store = FileSystemEventStore::new(log_folder_path);

        clock.advance(Duration::from_secs(10));
        let event = DomainEvent {
            meta: DomainEventMeta {
                aggregate_id: "123".to_string(),
                created_at: clock.now(),
            },
            payload: DomainEventPayload::Bookmark(BookmarkEventPayload::Created {
                url: "https://example.com".to_string(),
                title: "Example".to_string(),
            }),
        };

        event_store.store_event(event).unwrap();

        let event_file = temp.child("10000.json");
        event_file.assert(
            r#"{
  "meta": {
    "aggregate_id": "123",
    "created_at": {
      "secs_since_epoch": 10,
      "nanos_since_epoch": 0
    }
  },
  "payload": {
    "type": "bookmark",
    "event": "created",
    "url": "https://example.com",
    "title": "Example"
  }
}"#,
        );

        temp.close().unwrap();
    }

    #[test]
    fn test_events_are_read_from_disk_upon_instantiation() {
        let temp = TempDir::new().unwrap();
        let log_folder_path = temp.path().as_os_str();

        std::fs::write(
            Path::new(log_folder_path).join("10000.json"),
            r#"{
  "meta": {
    "aggregate_id": "123",
    "created_at": {
      "secs_since_epoch": 10,
      "nanos_since_epoch": 0
    }
  },
  "payload": {
    "type": "bookmark",
    "event": "created",
    "url": "https://example.com",
    "title": "Example"
  }
}"#,
        )
        .unwrap();

        std::fs::write(
            Path::new(log_folder_path).join("15000.json"),
            r#"{
  "meta": {
    "aggregate_id": "456",
    "created_at": {
      "secs_since_epoch": 15,
      "nanos_since_epoch": 0
    }
  },
  "payload": {
    "type": "bookmark",
    "event": "created",
    "url": "https://google.com",
    "title": "Google"
  }
}"#,
        )
        .unwrap();

        let es = FileSystemEventStore::new(log_folder_path);
        let events = es.get_events_for_aggregate("123".to_string());

        let clock = FakeClock::new();
        clock.advance(Duration::from_secs(10));

        assert_eq!(1, events.len());
        assert_eq!(
            events.get(0).unwrap(),
            &DomainEvent {
                meta: DomainEventMeta {
                    aggregate_id: "123".to_owned(),
                    created_at: clock.now()
                },
                payload: DomainEventPayload::Bookmark(BookmarkEventPayload::Created {
                    url: "https://example.com".to_string(),
                    title: "Example".to_string()
                })
            }
        )
    }
}
