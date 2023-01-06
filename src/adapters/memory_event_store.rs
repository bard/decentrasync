use crate::app::{
    Bookmark, BookmarkId, BookmarkQuery, DomainEvent, DomainEventPayload, EventStore,
};
use std::collections::HashMap;
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
    fn import_event(&self, event: DomainEvent) {
        match self.events.lock() {
            Ok(mut lock) => {
                lock.push(event);
                lock.sort_by(|a, b| a.meta.created_at.cmp(&b.meta.created_at));
            }
            _ => panic!(),
        }
    }

    fn push_event(&self, event: DomainEvent) {
        match self.events.lock() {
            Ok(mut lock) => lock.push(event),
            _ => panic!(),
        }
    }

    fn read_bookmarks(&self) -> Option<Vec<Bookmark>> {
        match self.events.lock() {
            Ok(lock) => {
                let mut map: HashMap<BookmarkId, Bookmark> = HashMap::new();
                lock.iter().for_each(|event| match &event.payload {
                    DomainEventPayload::BookmarkCreated { id, url, title } => {
                        if map.contains_key(id) {
                            panic!()
                        } else {
                            map.insert(
                                id.to_string(),
                                Bookmark {
                                    id: id.to_string(),
                                    url: url.to_string(),
                                    title: title.to_string(),
                                },
                            );
                        }
                    }
                    DomainEventPayload::BookmarkDeleted { id } => {
                        map.remove(id);
                    }
                    DomainEventPayload::BookmarkTitleUpdated { id, title } => match map.get(id) {
                        Some(bookmark) => {
                            map.insert(
                                id.clone(),
                                Bookmark {
                                    id: bookmark.id.clone(),
                                    url: bookmark.url.clone(),
                                    title: title.clone(),
                                },
                            );
                        }
                        None => {}
                    },
                });

                let mut items: Vec<Bookmark> = map.values().cloned().collect();
                items.sort_unstable_by_key(|b| b.id.clone());
                return Some(items);
            }
            _ => panic!(),
        }
    }

    fn read_bookmark(&self, query: &BookmarkQuery) -> Option<Bookmark> {
        match self.events.lock() {
            Ok(lock) => lock.iter().fold(None, |acc, event| match &event.payload {
                DomainEventPayload::BookmarkCreated { id, url, title } => {
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
                DomainEventPayload::BookmarkDeleted { id } => {
                    if id == &query.id {
                        None
                    } else {
                        acc
                    }
                }
                DomainEventPayload::BookmarkTitleUpdated { id, title } => {
                    if id == &query.id {
                        match acc {
                            Some(Bookmark {
                                id,
                                url,
                                title: _title,
                            }) => Some(Bookmark {
                                id,
                                title: title.clone(),
                                url,
                            }),
                            _ => None,
                        }
                    } else {
                        None
                    }
                }
            }),
            _ => panic!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use crate::app::DomainEventMeta;

    use super::*;
    use mock_instant::{Instant, MockClock};

    #[test]
    fn test_read_model_exposes_bookmark_by_id() {
        let store = MemoryEventStore::new();
        store.push_event(DomainEvent {
            meta: DomainEventMeta {
                created_at: Instant::now(),
            },
            payload: DomainEventPayload::BookmarkCreated {
                id: String::from("123"),
                url: String::from("https://example.com"),
                title: String::from("Example"),
            },
        });

        let bookmark = store
            .read_bookmark(&BookmarkQuery {
                id: "123".to_string(),
            })
            .unwrap();

        assert_eq!(bookmark.url, "https://example.com");
    }

    #[test]
    fn test_importing_an_event_sorts_the_log() {
        let store = MemoryEventStore::new();

        let earlier_external_event_time = Instant::now();
        let earlier_external_event = DomainEvent {
            meta: DomainEventMeta {
                created_at: earlier_external_event_time,
            },
            payload: DomainEventPayload::BookmarkCreated {
                id: String::from("abc"),
                url: String::from("https://google.com"),
                title: String::from("Google"),
            },
        };

        MockClock::advance(Duration::from_secs(10));

        store.push_event(DomainEvent {
            meta: DomainEventMeta {
                created_at: Instant::now(),
            },
            payload: DomainEventPayload::BookmarkCreated {
                id: String::from("123"),
                url: String::from("https://example.com"),
                title: String::from("Example"),
            },
        });

        store.import_event(earlier_external_event);

        let events = store.events.lock().unwrap();

        assert_eq!(events[0].meta.created_at, earlier_external_event_time);
    }
}
