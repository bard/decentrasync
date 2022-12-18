use std::sync::Arc;

use uuid::Uuid;

pub type BookmarkId = String;

#[derive(std::fmt::Debug, PartialEq, Clone)]
pub enum DomainEvent {
    BookmarkCreated {
        id: BookmarkId,
        url: String,
        title: String,
    },
    BookmarkDeleted {
        id: BookmarkId,
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
    fn read_bookmark(&self, query: &BookmarkQuery) -> Option<Bookmark>;
    fn save_bookmark(&self, bookmark: &Bookmark) -> Result<String, ()>;
    fn delete_bookmark(&self, query: &BookmarkQuery) -> ();
}

pub mod query {
    use super::*;

    pub fn read_bookmark(query: BookmarkQuery, store: Arc<dyn EventStore>) -> Option<Bookmark> {
        store.read_bookmark(&query)
    }
}

pub mod command {
    use super::*;

    pub fn delete_bookmark(query: BookmarkQuery, store: Arc<dyn EventStore>) -> () {
        store.delete_bookmark(&query)
    }

    pub fn create_bookmark(
        input: BookmarkInput,
        store: Arc<dyn EventStore>,
    ) -> Result<BookmarkId, ()> {
        let bookmark = Bookmark {
            id: Uuid::new_v4().to_string(),
            url: input.url.clone(),
            title: input.title.clone(),
        };

        store.save_bookmark(&bookmark)
    }
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use crate::adapters::memory_event_store::MemoryEventStore;

    use super::*;

    #[test]
    fn test_created_bookmark_can_be_retrieved() {
        let store = Arc::new(MemoryEventStore::new());
        let id = command::create_bookmark(
            BookmarkInput {
                url: "http://bar".to_string(),
                title: "bar".to_string(),
            },
            store.clone(),
        )
        .unwrap();

        assert!(Uuid::parse_str(id.as_str()).is_ok());

        let bookmark =
            query::read_bookmark(BookmarkQuery { id: id.clone() }, store.clone()).unwrap();

        assert_eq!(
            bookmark,
            Bookmark {
                id,
                url: "http://bar".to_string(),
                title: "bar".to_string(),
            }
        )
    }

    #[test]
    fn test_deleted_bookmark_cannot_be_retrieved() {
        let store = Arc::new(MemoryEventStore::new());
        let id = command::create_bookmark(
            BookmarkInput {
                url: "http://bar".to_string(),
                title: "bar".to_string(),
            },
            store.clone(),
        )
        .unwrap();

        command::delete_bookmark(BookmarkQuery { id: id.clone() }, store.clone());

        let bookmark = query::read_bookmark(BookmarkQuery { id: id.clone() }, store.clone());

        assert_eq!(bookmark, None)
    }
}
