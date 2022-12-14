use std::sync::Arc;

pub type BookmarkId = String;

#[derive(std::fmt::Debug, PartialEq, Clone)]
pub struct Bookmark {
    pub id: BookmarkId,
    pub url: String,
    pub title: String,
}

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

#[derive(std::fmt::Debug)]
pub struct BookmarkInput {
    pub url: String,
    pub title: String,
}

#[derive(std::fmt::Debug)]
pub struct BookmarkQuery {
    pub id: BookmarkId,
}

pub trait EventStore: Sync {
    fn push(&self, event: DomainEvent) -> Result<(), ()>;
    fn get_bookmark(&self, query: &BookmarkQuery) -> Option<Bookmark>;
    fn create_bookmark(&self, input: &BookmarkInput) -> Result<String, ()>;
    fn delete_bookmark(&self, query: &BookmarkQuery) -> ();
}

pub fn get_bookmark(query: BookmarkQuery, store: Arc<dyn EventStore>) -> Option<Bookmark> {
    store.get_bookmark(&query)
}

pub fn delete_bookmark(query: BookmarkQuery, store: Arc<dyn EventStore>) -> () {
    store.delete_bookmark(&query)
}

pub fn create_bookmark(input: BookmarkInput, store: Arc<dyn EventStore>) -> Result<BookmarkId, ()> {
    store.create_bookmark(&input)
}

#[cfg(test)]
mod tests {
    use crate::adapters::memory_event_store::MemoryEventStore;

    use super::*;

    #[test]
    fn test_created_bookmark_can_be_retrieved() {
        let store = Arc::new(MemoryEventStore::new());
        let id = create_bookmark(
            BookmarkInput {
                url: "http://bar".to_string(),
                title: "bar".to_string(),
            },
            store.clone(),
        )
        .unwrap();

        let bookmark = get_bookmark(BookmarkQuery { id: id.clone() }, store.clone()).unwrap();

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
        let id = create_bookmark(
            BookmarkInput {
                url: "http://bar".to_string(),
                title: "bar".to_string(),
            },
            store.clone(),
        )
        .unwrap();

        delete_bookmark(BookmarkQuery { id: id.clone() }, store.clone());

        let bookmark = get_bookmark(BookmarkQuery { id: id.clone() }, store.clone());

        assert_eq!(bookmark, None)
    }
}
