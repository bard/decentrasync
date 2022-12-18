use std::sync::Arc;

pub type BookmarkId = String;

#[cfg(test)]
use mock_instant::Instant;

#[cfg(not(test))]
use std::time::Instant;

#[derive(std::fmt::Debug, PartialEq, Clone)]
pub struct DomainEvent {
    pub meta: DomainEventMeta,
    pub payload: DomainEventPayload,
}

#[derive(std::fmt::Debug, PartialEq, Clone)]
pub struct DomainEventMeta {
    pub created_at: Instant,
}

#[derive(std::fmt::Debug, PartialEq, Clone)]
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

#[derive(std::fmt::Debug, PartialEq, Clone)]
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
    fn push_event(&self, event: DomainEvent) -> ();
    fn import_event(&self, event: DomainEvent) -> ();
    fn read_bookmark(&self, query: &BookmarkQuery) -> Option<Bookmark>;
}

pub mod query {
    use super::*;

    pub fn read_bookmark(query: BookmarkQuery, store: Arc<dyn EventStore>) -> Option<Bookmark> {
        store.read_bookmark(&query)
    }
}

pub mod command {
    use super::*;

    pub fn delete_bookmark(query: BookmarkQuery, store: Arc<dyn EventStore>) -> Result<(), ()> {
        store.push_event(DomainEvent {
            meta: DomainEventMeta {
                created_at: Instant::now(),
            },
            payload: DomainEventPayload::BookmarkDeleted {
                id: query.id.clone(),
            },
        });
        Ok(())
    }

    pub fn create_bookmark(bookmark: Bookmark, store: Arc<dyn EventStore>) -> Result<(), ()> {
        store.push_event(DomainEvent {
            meta: DomainEventMeta {
                created_at: Instant::now(),
            },
            payload: DomainEventPayload::BookmarkCreated {
                id: bookmark.id.clone(),
                url: bookmark.url.clone(),
                title: bookmark.title.clone(),
            },
        });
        Ok(())
    }

    pub fn update_bookmark_title(
        id: BookmarkId,
        title: String,
        store: Arc<dyn EventStore>,
    ) -> Result<(), ()> {
        let bookmark = store.read_bookmark(&BookmarkQuery { id }).unwrap();
        if bookmark.title != title {
            store.push_event(DomainEvent {
                meta: DomainEventMeta {
                    created_at: Instant::now(),
                },
                payload: DomainEventPayload::BookmarkTitleUpdated {
                    id: bookmark.id.clone(),
                    title: title.clone(),
                },
            });
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adapters::memory_event_store::MemoryEventStore;

    #[test]
    fn test_created_bookmark_can_be_retrieved() {
        let store = Arc::new(MemoryEventStore::new());

        command::create_bookmark(
            Bookmark {
                id: "123".to_string(),
                url: "http://bar".to_string(),
                title: "bar".to_string(),
            },
            store.clone(),
        )
        .unwrap();

        let bookmark = query::read_bookmark(
            BookmarkQuery {
                id: "123".to_string(),
            },
            store.clone(),
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
    fn test_bookmark_title_can_be_updated() {
        let store = Arc::new(MemoryEventStore::new());

        command::create_bookmark(
            Bookmark {
                id: "123".to_string(),
                url: "http://bar".to_string(),
                title: "bar".to_string(),
            },
            store.clone(),
        )
        .unwrap();

        command::update_bookmark_title("123".to_string(), "foo".to_string(), store.clone())
            .unwrap();

        let bookmark = query::read_bookmark(
            BookmarkQuery {
                id: "123".to_string(),
            },
            store.clone(),
        )
        .unwrap();

        assert_eq!(bookmark.title, "foo");
    }

    #[test]
    fn test_deleted_bookmark_cannot_be_retrieved() {
        let store = Arc::new(MemoryEventStore::new());
        command::create_bookmark(
            Bookmark {
                id: "123".to_string(),
                url: "http://bar".to_string(),
                title: "bar".to_string(),
            },
            store.clone(),
        )
        .unwrap();

        command::delete_bookmark(
            BookmarkQuery {
                id: "123".to_string(),
            },
            store.clone(),
        )
        .unwrap();

        let bookmark = query::read_bookmark(
            BookmarkQuery {
                id: "123".to_string(),
            },
            store.clone(),
        );

        assert_eq!(bookmark, None)
    }
}
