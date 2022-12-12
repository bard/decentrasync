use crate::entities::Bookmark;
use crate::values::BookmarkQuery;
use std::sync::Mutex;

pub trait Repository: Sync {
    fn insert(&self, bookmark: Bookmark) -> Result<Bookmark, ()>;
    fn fetch_first(&self, query: BookmarkQuery) -> Result<Bookmark, ()>;
}

pub struct InMemoryRepository {
    bookmarks: Mutex<Vec<Bookmark>>,
}

impl InMemoryRepository {
    pub fn new() -> Self {
        let bookmarks: Mutex<Vec<Bookmark>> = Mutex::new(vec![]);
        Self { bookmarks }
    }
}

impl Repository for InMemoryRepository {
    fn insert(&self, bookmark: Bookmark) -> Result<Bookmark, ()> {
        let mut lock = match self.bookmarks.lock() {
            Ok(lock) => lock,
            _ => panic!(), //return Err(InsertError::Unknown),
        };

        lock.push(bookmark.clone());
        Ok(bookmark)
    }

    fn fetch_first(&self, query: BookmarkQuery) -> Result<Bookmark, ()> {
        let lock = match self.bookmarks.lock() {
            Ok(lock) => lock,
            _ => panic!(), //return Err(InsertError::Unknown),
        };

        match lock.iter().find(|b| b.id == query.id) {
            Some(bookmark) => Ok(bookmark.clone()), // XXX how to get rid of clone?
            None => Err(()),
        }
    }
}
