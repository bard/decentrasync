use crate::{app, ports::EventStore, ports::EventStoreError};
use std::{
    ffi::{OsStr, OsString},
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
    fn import_event(&self, _event: app::DomainEvent) -> Result<(), EventStoreError> {
        todo!()
    }

    fn store_event(&self, event: app::DomainEvent) -> Result<(), EventStoreError> {
        let timestamp_millis = event
            .meta
            .created_at
            .duration_since(UNIX_EPOCH)
            .map_err(|_source| EventStoreError::Generic)?
            .as_millis();

        let stored_event_path =
            Path::new(self.log_folder_path.as_os_str()).join(format!("{}.json", timestamp_millis));

        std::fs::write(stored_event_path, serde_json::to_string(&event).unwrap()).unwrap();

        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use std::sync::Arc;
    use std::time::Duration;

    use crate::adapters::clock::FakeClock;
    use crate::app::{DomainEventMeta, DomainEventPayload};
    use crate::ports::Clock;

    use super::*;
    use assert_fs::assert::PathAssert;
    use assert_fs::fixture::PathChild;
    use assert_fs::TempDir;

    #[test]
    fn test_storing_an_event_writes_to_disk() {
        let temp = TempDir::new().unwrap();
        let log_folder_path = temp.path().as_os_str();
        let clock = Arc::new(FakeClock::new());
        let event_store = FileSystemEventStore::new(log_folder_path);

        clock.advance(Duration::from_secs(10));
        let event = app::DomainEvent {
            meta: DomainEventMeta {
                created_at: clock.now(),
            },
            payload: DomainEventPayload::BookmarkCreated {
                id: "123".to_string(),
                url: "https://example.com".to_string(),
                title: "Example".to_string(),
            },
        };

        event_store.store_event(event).unwrap();

        let event_file = temp.child("10000.json");
        event_file.assert("{\"meta\":{\"created_at\":{\"secs_since_epoch\":10,\"nanos_since_epoch\":0}},\"payload\":{\"BookmarkCreated\":{\"id\":\"123\",\"url\":\"https://example.com\",\"title\":\"Example\"}}}");
        temp.close().unwrap();
    }
}
