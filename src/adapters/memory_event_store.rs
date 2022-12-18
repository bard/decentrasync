use crate::app::{Bookmark, BookmarkQuery, DomainEvent, DomainEventEnvelope, EventStore};
use std::sync::Mutex;

pub struct MemoryEventStore {
    events: Mutex<Vec<DomainEventEnvelope>>,
}

impl MemoryEventStore {
    pub fn new() -> Self {
        let events: Mutex<Vec<DomainEventEnvelope>> = Mutex::new(vec![]);
        Self { events }
    }

    // pub fn external_event_received(&self, event: DomainEventEnvelope) -> Result<(), ()> {
    //     match self.events.lock() {
    //         Ok(mut lock) => {
    //             lock.push(event.clone());
    //             lock.sort_by(|a, b| b.time.cmp(&a.time));
    //             Ok(())
    //         }
    //         _ => panic!(),
    //     }
    // }
}

impl EventStore for MemoryEventStore {
    fn push_event(&self, event: DomainEventEnvelope) -> () {
        match self.events.lock() {
            Ok(mut lock) => lock.push(event.clone()),
            _ => panic!(),
        }
    }

    fn read_bookmark(&self, query: &BookmarkQuery) -> Option<Bookmark> {
        match self.events.lock() {
            Ok(lock) => lock.iter().fold(None, |acc, event| match &event.payload {
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
                DomainEvent::BookmarkTitleUpdated { id, title } => {
                    if id == &query.id {
                        match acc {
                            Some(Bookmark {
                                id,
                                url,
                                title: _title,
                            }) => Some(Bookmark {
                                id: id.clone(),
                                title: title.clone(),
                                url: url.clone(),
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
    use super::*;
    use mock_instant::Instant;

    #[test]
    fn test_read_model_exposes_bookmark_by_id() {
        let store = MemoryEventStore::new();
        store.push_event(DomainEventEnvelope {
            time: Instant::now(),
            payload: DomainEvent::BookmarkCreated {
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
    fn test_external_events_are_received_and_inserted_according_to_their_timestamp() {}
}
