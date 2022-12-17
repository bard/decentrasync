use crate::app::{Bookmark, BookmarkId, BookmarkQuery, DomainEvent, EventStore};
use std::sync::Mutex;

pub struct MemoryEventStore {
    events: Mutex<Vec<DomainEvent>>,
}

impl MemoryEventStore {
    pub fn new() -> Self {
        let events: Mutex<Vec<DomainEvent>> = Mutex::new(vec![]);
        Self { events }
    }
}

impl EventStore for MemoryEventStore {
    fn read_bookmark(&self, query: &BookmarkQuery) -> Option<Bookmark> {
        match self.events.lock() {
            Ok(lock) => lock.iter().fold(None, |acc, event| match event {
                DomainEvent::BookmarkCreated { id, url, title } => {
                    if id == &query.id {
                        Some(Bookmark {
                            id: id.clone(),
                            title: title.clone(),
                            url: url.clone(),
                        })
                    } else {
                        acc
                    }
                }
                DomainEvent::BookmarkDeleted { id } => {
                    if id == &query.id {
                        None
                    } else {
                        acc
                    }
                }
            }),
            _ => panic!(),
        }
    }

    fn save_bookmark(&self, bookmark: &Bookmark) -> Result<BookmarkId, ()> {
        match self.events.lock() {
            Ok(mut lock) => {
                lock.push(DomainEvent::BookmarkCreated {
                    id: bookmark.id.clone(),
                    url: bookmark.url.clone(),
                    title: bookmark.title.clone(),
                });
                Ok(bookmark.id.clone())
            }
            _ => panic!(),
        }
    }

    fn delete_bookmark(&self, query: &BookmarkQuery) -> () {
        match self.events.lock() {
            Ok(mut lock) => {
                lock.push(DomainEvent::BookmarkDeleted {
                    id: query.id.clone(),
                });
            }
            _ => panic!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation_causes_event_to_be_added_to_log() {
        let store = MemoryEventStore::new();
        store
            .save_bookmark(&Bookmark {
                id: String::from("123"),
                url: String::from("https://example.com"),
                title: String::from("Example"),
            })
            .unwrap();

        assert_eq!(store.events.lock().unwrap().len(), 1);
    }
}
