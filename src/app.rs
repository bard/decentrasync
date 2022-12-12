use std::sync::Arc;

#[derive(std::fmt::Debug, PartialEq, Clone)]
pub struct Bookmark {
    pub id: String,
    pub url: String,
    pub title: String,
}

#[derive(std::fmt::Debug, PartialEq, Clone)]
pub struct BookmarkCreatedEvent {
    pub id: String,
    pub url: String,
    pub title: String,
}

type BookmarkId = String;

#[derive(std::fmt::Debug)]
pub struct BookmarkInput {
    pub url: String,
    pub title: String,
}

#[derive(std::fmt::Debug)]
pub struct BookmarkQuery {
    pub id: BookmarkId,
}

pub trait Repository: Sync {
    fn insert(&self, bookmark: Bookmark) -> Result<Bookmark, ()>;
    fn fetch_first(&self, query: BookmarkQuery) -> Result<Bookmark, ()>;
}

pub trait EventLog: Sync {
    fn push(&self, event: BookmarkCreatedEvent) -> Result<(), ()>;
}

pub fn read_bookmark(
    bookmark_query: BookmarkQuery,
    repo: Arc<dyn Repository>,
) -> Result<Bookmark, ()> {
    repo.fetch_first(bookmark_query)
}

pub fn delete_bookmark(_bookmark_query: BookmarkQuery) -> () {}

pub fn create_bookmark(
    bookmark_input: BookmarkInput,
    repo: Arc<dyn Repository>,
    log: Arc<dyn EventLog>,
) -> Result<String, ()> {
    let _repo_res = repo.insert(Bookmark {
        id: bookmark_input.url.clone(), // XXX temporary
        url: bookmark_input.url.clone(),
        title: bookmark_input.title.clone(),
    });

    let _log_res = log.push(BookmarkCreatedEvent {
        id: bookmark_input.url.clone(), // XXX temporary
        url: bookmark_input.url.clone(),
        title: bookmark_input.title.clone(),
    });

    Ok(bookmark_input.url)
}

#[cfg(test)]
mod tests {
    use crate::adapters::{memory_event_log::MemoryEventLog, memory_repository::MemoryRepository};

    use super::*;

    #[test]
    fn test_created_bookmark_is_saved_in_repository() {
        let repo = Arc::new(MemoryRepository::new());
        let log = Arc::new(MemoryEventLog::new());
        let id = create_bookmark(
            BookmarkInput {
                url: "http://bar".to_string(),
                title: "bar".to_string(),
            },
            repo.clone(),
            log.clone(),
        )
        .unwrap();

        let bookmark = read_bookmark(BookmarkQuery { id: id.clone() }, repo.clone()).unwrap();

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
