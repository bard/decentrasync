use std::sync::Mutex;

use crate::app::{Bookmark, BookmarkQuery, Repository};

pub struct MemoryRepository {
    bookmarks: Mutex<Vec<Bookmark>>,
}

impl MemoryRepository {
    pub fn new() -> Self {
        let bookmarks: Mutex<Vec<Bookmark>> = Mutex::new(vec![]);
        Self { bookmarks }
    }
}

impl Repository for MemoryRepository {
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
