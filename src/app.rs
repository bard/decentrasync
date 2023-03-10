use crate::{
    domain::aggregates::BookmarkAggregate,
    domain::commands::BookmarkCommand,
    domain::errors::DomainError,
    domain::{
        data::{Aggregate, BookmarkData, DomainEvent, DomainEventMeta},
        events::DomainEventPayload,
    },
    ports::{Clock, EventStore, ReadModel},
};
use std::sync::Arc;

pub fn init(event_store: Arc<dyn EventStore>, read_model: Arc<dyn ReadModel>) {
    for event in event_store.events_iter() {
        read_model.update(&event).unwrap(); // XXX handle error
    }
}

pub fn read_bookmark(id: &str, read_model: Arc<dyn ReadModel>) -> Option<BookmarkData> {
    read_model.read_bookmark(id)
}

pub fn read_bookmarks(read_model: Arc<dyn ReadModel>) -> Option<Vec<BookmarkData>> {
    read_model.read_bookmarks()
}

pub fn delete_bookmark(
    id: &str,
    event_store: Arc<dyn EventStore>,
    read_model: Arc<dyn ReadModel>,
    clock: Arc<dyn Clock>,
) -> Result<(), DomainError> {
    let bookmark = event_store.get_events_for_aggregate(&id).iter().fold(
        BookmarkAggregate::new(&id),
        |aggr, ref evt| match &evt.payload {
            DomainEventPayload::Bookmark(payload) => aggr.apply_event(&payload, &evt.meta),
            _ => aggr,
        },
    );

    let event_payload = bookmark.handle_command(&BookmarkCommand::Delete)?;

    let event = DomainEvent {
        meta: DomainEventMeta {
            aggregate_id: id.to_owned(),
            created_at: clock.now(),
        },
        payload: DomainEventPayload::Bookmark(event_payload),
    };

    event_store
        .store_event(event.clone())
        .map_err(|_source| DomainError::PortError)?;
    read_model
        .update(&event)
        .map_err(|_source| DomainError::PortError)?;

    Ok(())
}

pub fn create_bookmark(
    id: &str,
    url: &str,
    title: &str,
    event_store: Arc<dyn EventStore>,
    read_model: Arc<dyn ReadModel>,
    clock: Arc<dyn Clock>,
) -> Result<(), DomainError> {
    let bookmark =
        event_store
            .get_events_for_aggregate(&id)
            .iter()
            .fold(BookmarkAggregate::new(&id), {
                |aggr, ref evt| match &evt.payload {
                    DomainEventPayload::Bookmark(payload) => aggr.apply_event(&payload, &evt.meta),
                    _ => aggr,
                }
            });

    let command = BookmarkCommand::BookmarkPage {
        url: url.to_owned(),
        title: title.to_owned(),
    };
    let event_payload = bookmark.handle_command(&command)?;

    let event = DomainEvent {
        meta: DomainEventMeta {
            aggregate_id: id.to_owned(),
            created_at: clock.now(),
        },
        payload: DomainEventPayload::Bookmark(event_payload),
    };

    event_store
        .store_event(event.clone())
        .map_err(|_source| DomainError::PortError)?;
    read_model
        .update(&event)
        .map_err(|_source| DomainError::PortError)?;

    Ok(())
}

pub fn update_bookmark_title(
    id: &str,
    title: &str,
    event_store: Arc<dyn EventStore>,
    read_model: Arc<dyn ReadModel>,
    clock: Arc<dyn Clock>,
) -> Result<(), DomainError> {
    let bookmark = event_store.get_events_for_aggregate(&id).iter().fold(
        BookmarkAggregate::new(&id),
        |aggr, ref evt| match &evt.payload {
            DomainEventPayload::Bookmark(payload) => aggr.apply_event(&payload, &evt.meta),
            _ => aggr,
        },
    );

    let command = BookmarkCommand::UpdateTitle {
        title: title.to_owned(),
    };
    let event_payload = bookmark.handle_command(&command)?;

    let event = DomainEvent {
        meta: DomainEventMeta {
            aggregate_id: id.to_owned(),
            created_at: clock.now(),
        },
        payload: DomainEventPayload::Bookmark(event_payload),
    };

    // TODO no error should be returned here since command has been
    // already validated
    event_store
        .store_event(event.clone())
        .map_err(|_source| DomainError::PortError)?;
    read_model
        .update(&event)
        .map_err(|_source| DomainError::PortError)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        adapters::{
            clock::FakeClock, memory_event_store::MemoryEventStore,
            memory_read_model::MemoryReadModel,
        },
        domain::data::BookmarkData,
    };

    #[test]
    fn test_created_bookmark_can_be_retrieved() {
        let event_store = Arc::new(MemoryEventStore::new());
        let read_model = Arc::new(MemoryReadModel::new());
        let clock = Arc::new(FakeClock::new());

        create_bookmark(
            "123",
            "http://bar",
            "bar",
            event_store.clone(),
            read_model.clone(),
            clock.clone(),
        )
        .unwrap();

        let bookmark = read_bookmark("123", read_model.clone()).unwrap();

        assert_eq!(
            bookmark,
            BookmarkData {
                id: "123".to_owned(),
                url: "http://bar".to_owned(),
                title: "bar".to_owned(),
            }
        )
    }

    #[test]
    fn test_bookmark_list_can_be_retrieved() {
        let event_store = Arc::new(MemoryEventStore::new());
        let read_model = Arc::new(MemoryReadModel::new());
        let clock = Arc::new(FakeClock::new());

        create_bookmark(
            "123",
            "http://bar",
            "bar",
            event_store.clone(),
            read_model.clone(),
            clock.clone(),
        )
        .unwrap();

        create_bookmark(
            "456",
            "http://foo",
            "foo",
            event_store.clone(),
            read_model.clone(),
            clock.clone(),
        )
        .unwrap();

        update_bookmark_title(
            "456",
            "foobar",
            event_store.clone(),
            read_model.clone(),
            clock.clone(),
        )
        .unwrap();

        let bookmarks = read_bookmarks(read_model.clone()).unwrap();

        assert_eq!(bookmarks.len(), 2);
        assert_eq!(bookmarks[0].id, "123");
        assert_eq!(bookmarks[1].id, "456");
        assert_eq!(bookmarks[1].title, "foobar");
    }

    #[test]
    fn test_bookmark_title_can_be_updated() {
        let event_store = Arc::new(MemoryEventStore::new());
        let read_model = Arc::new(MemoryReadModel::new());
        let clock = Arc::new(FakeClock::new());

        create_bookmark(
            "123",
            "http://bar",
            "bar",
            event_store.clone(),
            read_model.clone(),
            clock.clone(),
        )
        .unwrap();

        update_bookmark_title(
            "123",
            "foo",
            event_store.clone(),
            read_model.clone(),
            clock.clone(),
        )
        .unwrap();

        let bookmark = read_bookmark("123", read_model.clone()).unwrap();

        assert_eq!(bookmark.title, "foo");
    }

    #[test]
    fn test_deleted_bookmark_cannot_be_retrieved() {
        let event_store = Arc::new(MemoryEventStore::new());
        let read_model = Arc::new(MemoryReadModel::new());
        let clock = Arc::new(FakeClock::new());

        create_bookmark(
            "123",
            "http://bar",
            "bar",
            event_store.clone(),
            read_model.clone(),
            clock.clone(),
        )
        .unwrap();

        delete_bookmark(
            "123",
            event_store.clone(),
            read_model.clone(),
            clock.clone(),
        )
        .unwrap();

        let bookmark = read_bookmark("123", read_model.clone());

        assert_eq!(bookmark, None)
    }

    #[test]
    fn test_deleting_non_existent_bookmark_is_rejected() {
        let event_store = Arc::new(MemoryEventStore::new());
        let read_model = Arc::new(MemoryReadModel::new());
        let clock = Arc::new(FakeClock::new());

        let err = delete_bookmark(
            "123",
            event_store.clone(),
            read_model.clone(),
            clock.clone(),
        )
        .unwrap_err();

        assert_eq!(err, DomainError::NoSuchBookmark);
    }
}
