use std::sync::Mutex;

use crate::app::{Bookmark, BookmarkId, BookmarkInput, BookmarkQuery, DomainEvent, EventStore};

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
    fn push(&self, event: DomainEvent) -> Result<(), ()> {
        let mut lock = match self.events.lock() {
            Ok(lock) => lock,
            _ => panic!(),
        };

        lock.push(event.clone());
        Ok(())
    }

    fn get_bookmark(&self, query: &BookmarkQuery) -> Option<Bookmark> {
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
            }),
            _ => panic!(),
        }
    }

    fn create_bookmark(&self, input: &BookmarkInput) -> Result<BookmarkId, ()> {
        match self.events.lock() {
            Ok(mut lock) => {
                lock.push(DomainEvent::BookmarkCreated {
                    id: input.url.clone(), // XXX temporary
                    url: input.url.clone(),
                    title: input.title.clone(),
                });

                Ok(input.url.clone())
            }
            _ => panic!(),
        }
    }
}
