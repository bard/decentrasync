use crate::app::{Bookmark, BookmarkId, BookmarkQuery, DomainEvent, DomainEventPayload, ReadModel};
use std::{collections::HashMap, sync::Mutex};

pub struct MemoryReadModel {
    bookmarks_by_id: Mutex<HashMap<BookmarkId, Bookmark>>,
}

impl MemoryReadModel {
    pub fn new() -> Self {
        let bookmarks_by_id: Mutex<HashMap<BookmarkId, Bookmark>> = Mutex::new(HashMap::new());
        Self { bookmarks_by_id }
    }
}

impl ReadModel for MemoryReadModel {
    fn update(&self, event: &DomainEvent) {
        let mut bookmarks_by_id = self.bookmarks_by_id.lock().unwrap();

        match &event.payload {
            DomainEventPayload::BookmarkCreated { id, url, title } => {
                if bookmarks_by_id.contains_key(id) {
                    panic!()
                } else {
                    bookmarks_by_id.insert(
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
                bookmarks_by_id.remove(id);
            }
            DomainEventPayload::BookmarkTitleUpdated { id, title } => {
                bookmarks_by_id
                    .entry(id.to_string())
                    .and_modify(|bookmark| {
                        *bookmark = Bookmark {
                            id: bookmark.id.clone(),
                            url: bookmark.url.clone(),
                            title: title.clone(),
                        }
                    });
            }
        }
    }

    fn read_bookmarks(&self) -> Option<Vec<Bookmark>> {
        let bookmarks_by_id = self.bookmarks_by_id.lock().unwrap();
        let mut items: Vec<Bookmark> = bookmarks_by_id.values().cloned().collect();
        items.sort_unstable_by_key(|b| b.id.clone());
        return Some(items);
    }

    fn read_bookmark(&self, query: &BookmarkQuery) -> Option<Bookmark> {
        let bookmarks_by_id = self.bookmarks_by_id.lock().unwrap();
        match bookmarks_by_id.get(&query.id) {
            Some(bookmark) => Some(bookmark.clone()),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        adapters::clock::FakeClock,
        app::{DomainEventMeta, Clock},
    };

    #[test]
    fn test_read_model_exposes_bookmark_by_id() {
        let read_model = MemoryReadModel::new();
        let clock = FakeClock::new();

        read_model.update(&DomainEvent {
            meta: DomainEventMeta {
                created_at: clock.now(),
            },
            payload: DomainEventPayload::BookmarkCreated {
                id: String::from("123"),
                url: String::from("https://example.com"),
                title: String::from("Example"),
            },
        });

        let bookmark = read_model
            .read_bookmark(&BookmarkQuery {
                id: "123".to_string(),
            })
            .unwrap();

        assert_eq!(bookmark.url, "https://example.com");
    }
}
