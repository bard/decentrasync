use serde::Serialize;
use std::{sync::Arc, time::SystemTime};

pub type BookmarkId = String;

#[derive(std::fmt::Debug, PartialEq, Eq, Clone, Serialize)]
pub struct DomainEvent {
    pub meta: DomainEventMeta,
    pub payload: DomainEventPayload,
}

#[derive(std::fmt::Debug, PartialEq, Eq, Clone, Serialize)]
pub struct DomainEventMeta {
    pub created_at: SystemTime,
}

#[derive(std::fmt::Debug, PartialEq, Eq, Clone, Serialize)]
pub enum DomainEventPayload {
    BookmarkCreated {
        id: BookmarkId,
        url: String,
        title: String,
    },
    BookmarkDeleted {
        id: BookmarkId,
    },
    BookmarkTitleUpdated {
        id: BookmarkId,
        title: String,
    },
}

#[derive(std::fmt::Debug, PartialEq, Eq, Clone)]
pub struct Bookmark {
    pub id: BookmarkId,
    pub url: String,
    pub title: String,
}

#[derive(std::fmt::Debug)]
pub struct BookmarkInput {
    pub url: String,
    pub title: String,
}

#[derive(std::fmt::Debug)]
pub struct BookmarkQuery {
    pub id: BookmarkId,
}

pub trait EventStore: Send + Sync {
    fn store_event(&self, event: DomainEvent);
    fn import_event(&self, event: DomainEvent);
}

pub trait ReadModel: Send + Sync {
    fn update(&self, event: &DomainEvent);
    fn read_bookmark(&self, query: &BookmarkQuery) -> Option<Bookmark>;
    fn read_bookmarks(&self) -> Option<Vec<Bookmark>>;
}

pub trait Clock: Send + Sync {
    fn now(&self) -> SystemTime;
}

pub mod query {
    use super::*;

    pub fn read_bookmark(query: BookmarkQuery, read_model: Arc<dyn ReadModel>) -> Option<Bookmark> {
        read_model.read_bookmark(&query)
    }

    pub fn read_bookmarks(read_model: Arc<dyn ReadModel>) -> Option<Vec<Bookmark>> {
        read_model.read_bookmarks()
    }
}

pub mod command {

    use super::*;

    pub fn delete_bookmark(
        id: BookmarkId,
        event_store: Arc<dyn EventStore>,
        read_model: Arc<dyn ReadModel>,
        clock: Arc<dyn Clock>,
    ) -> Result<(), ()> {
        let event = DomainEvent {
            meta: DomainEventMeta {
                created_at: clock.now(),
            },
            payload: DomainEventPayload::BookmarkDeleted { id },
        };

        read_model.update(&event);
        event_store.store_event(event);

        Ok(())
    }

    pub fn create_bookmark(
        bookmark: Bookmark,
        event_store: Arc<dyn EventStore>,
        read_model: Arc<dyn ReadModel>,
        clock: Arc<dyn Clock>,
    ) -> Result<(), ()> {
        let event = DomainEvent {
            meta: DomainEventMeta {
                created_at: clock.now(),
            },
            payload: DomainEventPayload::BookmarkCreated {
                id: bookmark.id.clone(),
                url: bookmark.url.clone(),
                title: bookmark.title,
            },
        };

        read_model.update(&event);
        event_store.store_event(event);

        Ok(())
    }

    pub fn update_bookmark_title(
        id: BookmarkId,
        title: String,
        event_store: Arc<dyn EventStore>,
        read_model: Arc<dyn ReadModel>,
        clock: Arc<dyn Clock>,
    ) -> Result<(), ()> {
        let bookmark = read_model.read_bookmark(&BookmarkQuery { id }).unwrap();
        if bookmark.title != title {
            let event = DomainEvent {
                meta: DomainEventMeta {
                    created_at: clock.now(),
                },
                payload: DomainEventPayload::BookmarkTitleUpdated {
                    id: bookmark.id,
                    title,
                },
            };

            read_model.update(&event);
            event_store.store_event(event);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adapters::{
        clock::FakeClock, memory_event_store::MemoryEventStore, memory_read_model::MemoryReadModel,
    };

    #[test]
    fn test_created_bookmark_can_be_retrieved() {
        let event_store = Arc::new(MemoryEventStore::new());
        let read_model = Arc::new(MemoryReadModel::new());
        let clock = Arc::new(FakeClock::new());

        command::create_bookmark(
            Bookmark {
                id: "123".to_string(),
                url: "http://bar".to_string(),
                title: "bar".to_string(),
            },
            event_store.clone(),
            read_model.clone(),
            clock.clone(),
        )
        .unwrap();

        let bookmark = query::read_bookmark(
            BookmarkQuery {
                id: "123".to_string(),
            },
            read_model.clone(),
        )
        .unwrap();

        assert_eq!(
            bookmark,
            Bookmark {
                id: "123".to_string(),
                url: "http://bar".to_string(),
                title: "bar".to_string(),
            }
        )
    }

    #[test]
    fn test_bookmark_list_can_be_retrieved() {
        let event_store = Arc::new(MemoryEventStore::new());
        let read_model = Arc::new(MemoryReadModel::new());
        let clock = Arc::new(FakeClock::new());

        command::create_bookmark(
            Bookmark {
                id: "123".to_string(),
                url: "http://bar".to_string(),
                title: "bar".to_string(),
            },
            event_store.clone(),
            read_model.clone(),
            clock.clone(),
        )
        .unwrap();

        command::create_bookmark(
            Bookmark {
                id: "456".to_string(),
                url: "http://foo".to_string(),
                title: "foo".to_string(),
            },
            event_store.clone(),
            read_model.clone(),
            clock.clone(),
        )
        .unwrap();

        command::update_bookmark_title(
            "456".to_string(),
            "foobar".to_string(),
            event_store.clone(),
            read_model.clone(),
            clock.clone(),
        )
        .unwrap();

        let bookmarks = query::read_bookmarks(read_model.clone()).unwrap();

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

        command::create_bookmark(
            Bookmark {
                id: "123".to_string(),
                url: "http://bar".to_string(),
                title: "bar".to_string(),
            },
            event_store.clone(),
            read_model.clone(),
            clock.clone(),
        )
        .unwrap();

        command::update_bookmark_title(
            "123".to_string(),
            "foo".to_string(),
            event_store.clone(),
            read_model.clone(),
            clock.clone(),
        )
        .unwrap();

        let bookmark = query::read_bookmark(
            BookmarkQuery {
                id: "123".to_string(),
            },
            read_model.clone(),
        )
        .unwrap();

        assert_eq!(bookmark.title, "foo");
    }

    #[test]
    fn test_deleted_bookmark_cannot_be_retrieved() {
        let event_store = Arc::new(MemoryEventStore::new());
        let read_model = Arc::new(MemoryReadModel::new());
        let clock = Arc::new(FakeClock::new());

        command::create_bookmark(
            Bookmark {
                id: "123".to_string(),
                url: "http://bar".to_string(),
                title: "bar".to_string(),
            },
            event_store.clone(),
            read_model.clone(),
            clock.clone(),
        )
        .unwrap();

        command::delete_bookmark(
            "123".to_string(),
            event_store.clone(),
            read_model.clone(),
            clock.clone(),
        )
        .unwrap();

        let bookmark = query::read_bookmark(
            BookmarkQuery {
                id: "123".to_string(),
            },
            read_model.clone(),
        );

        assert_eq!(bookmark, None)
    }
}
