use std::sync::Arc;

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

    pub fn create_bookmark(bookmark: Bookmark, store: Arc<dyn EventStore>) -> Result<(), ()> {
        store.save_bookmark(&bookmark).unwrap();
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
        );

        let bookmark = query::read_bookmark(
            BookmarkQuery {
                id: "123".to_string(),
            },
            store.clone(),
        );

        assert_eq!(bookmark, None)
    }
}
