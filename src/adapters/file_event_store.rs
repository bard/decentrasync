use crate::{domain::data::DomainEvent, ports::EventStore, ports::EventStoreError};
use std::{
    collections::VecDeque,
    ffi::{OsStr, OsString},
    fs,
    path::{Path, PathBuf},
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

impl Iterator for FilesystemEventStoreIterator {
    type Item = DomainEvent;

    fn next(&mut self) -> Option<Self::Item> {
        match self.sorted_event_filenames.pop_front() {
            Some(f) => {
                Some(serde_json::from_str::<DomainEvent>(&fs::read_to_string(f).unwrap()).unwrap())
            }
            None => None,
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

    fn get_events_for_aggregate(&self, aggregate_id: &str) -> Vec<DomainEvent> {
        self.events_iter()
            .filter(|e| e.meta.aggregate_id == aggregate_id)
            .collect::<Vec<DomainEvent>>()
    }

    fn events_iter(&self) -> Box<dyn Iterator<Item = DomainEvent>> {
        Box::new(FilesystemEventStoreIterator::new(&self.log_folder_path))
    }
}

struct FilesystemEventStoreIterator {
    sorted_event_filenames: VecDeque<PathBuf>,
}

impl FilesystemEventStoreIterator {
    pub fn new(log_folder_path: &OsStr) -> Self {
        let mut event_filenames = std::fs::read_dir(log_folder_path)
            .unwrap()
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<VecDeque<_>, std::io::Error>>()
            .unwrap();

        event_filenames.make_contiguous().sort_unstable();

        Self {
            sorted_event_filenames: event_filenames,
        }
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
    use std::time::{Duration, SystemTime};

    #[test]
    fn test_storing_an_event_writes_to_disk() {
        let temp = TempDir::new().unwrap();
        let log_folder_path = temp.path().as_os_str();
        let clock = Arc::new(FakeClock::new());
        let event_store = FileSystemEventStore::new(log_folder_path);

        clock.advance(Duration::from_secs(10));
        let event = DomainEvent {
            meta: DomainEventMeta {
                aggregate_id: "123".to_owned(),
                created_at: clock.now(),
            },
            payload: DomainEventPayload::Bookmark(BookmarkEventPayload::Created {
                url: "https://example.com".to_owned(),
                title: "Example".to_owned(),
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
    fn test_entire_log_of_events_can_be_read_from_disk_on_demand() {
        let temp = TempDir::new().unwrap();
        let log_folder_path = temp.path().as_os_str();
        let es = FileSystemEventStore::new(log_folder_path);

        setup_sample_log(log_folder_path);

        let events: Vec<DomainEvent> = es.events_iter().collect();
        assert_eq!(2, events.len());
        assert_eq!(
            events.get(0).unwrap(),
            &DomainEvent {
                meta: DomainEventMeta {
                    aggregate_id: "123".to_owned(),
                    created_at: SystemTime::UNIX_EPOCH + Duration::from_secs(10)
                },
                payload: DomainEventPayload::Bookmark(BookmarkEventPayload::Created {
                    url: "https://example.com".to_owned(),
                    title: "Example".to_owned()
                })
            }
        )
    }

    fn setup_sample_log(log_folder_path: &OsStr) {
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
    }
}
