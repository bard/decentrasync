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
}

pub fn get_bookmark(query: BookmarkQuery, store: Arc<dyn EventStore>) -> Option<Bookmark> {
    store.get_bookmark(&query)
}

pub fn delete_bookmark(_bookmark_query: BookmarkQuery) -> () {}

pub fn create_bookmark(input: BookmarkInput, store: Arc<dyn EventStore>) -> Result<BookmarkId, ()> {
    store.create_bookmark(&input)
}

#[cfg(test)]
mod tests {
    use crate::adapters::memory_event_store::MemoryEventStore;

    use super::*;

    #[test]
    fn test_created_bookmark_is_saved_in_log() {
        let log = Arc::new(MemoryEventStore::new());
        let id = create_bookmark(
            BookmarkInput {
                url: "http://bar".to_string(),
                title: "bar".to_string(),
            },
            log.clone(),
        )
        .unwrap();

        let bookmark = get_bookmark(BookmarkQuery { id: id.clone() }, log.clone()).unwrap();

        assert_eq!(
            bookmark,
            Bookmark {
                id,
                url: "http://bar".to_string(),
                title: "bar".to_string(),
            }
        )
    }
}
