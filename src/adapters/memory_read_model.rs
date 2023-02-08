use crate::domain::data::{BookmarkData, DomainEvent};
use crate::domain::events::{BookmarkEventPayload, DomainEventPayload};
use crate::ports::{ReadModel, ReadModelError};
use std::{collections::HashMap, sync::Mutex};

pub struct MemoryReadModel {
    bookmarks_by_id: Mutex<HashMap<String, BookmarkData>>,
}

impl MemoryReadModel {
    pub fn new() -> Self {
        let bookmarks_by_id: Mutex<HashMap<String, BookmarkData>> = Mutex::new(HashMap::new());
        Self { bookmarks_by_id }
    }
}

impl ReadModel for MemoryReadModel {
    fn update(&self, event: &DomainEvent) -> Result<(), ReadModelError> {
        let mut bookmarks_by_id = self.bookmarks_by_id.lock().unwrap();

        match &event.payload {
            DomainEventPayload::Bookmark(BookmarkEventPayload::Created { url, title }) => {
                if bookmarks_by_id.contains_key(&*event.meta.aggregate_id) {
                    Err(ReadModelError::Generic)
                } else {
                    bookmarks_by_id.insert(
                        event.meta.aggregate_id.to_owned(),
                        BookmarkData {
                            id: event.meta.aggregate_id.to_owned(),
                            url: url.to_owned(),
                            title: title.to_owned(),
                        },
                    );
                    Ok(())
                }
            }
            DomainEventPayload::Bookmark(BookmarkEventPayload::Deleted) => {
                bookmarks_by_id.remove(&*event.meta.aggregate_id);
                Ok(())
            }
            DomainEventPayload::Bookmark(BookmarkEventPayload::TitleUpdated { title }) => {
                bookmarks_by_id
                    .entry(event.meta.aggregate_id.to_owned())
                    .and_modify(|bookmark| {
                        *bookmark = BookmarkData {
                            id: bookmark.id.clone(),
                            url: bookmark.url.clone(),
                            title: title.clone(),
                        }
                    });
                Ok(())
            }
            _ => todo!(),
        }
    }

    fn read_bookmarks(&self) -> Option<Vec<BookmarkData>> {
        let bookmarks_by_id = self.bookmarks_by_id.lock().unwrap();
        let mut items: Vec<BookmarkData> = bookmarks_by_id.values().cloned().collect();
        items.sort_unstable_by_key(|b| b.id.clone());
        return Some(items);
    }

    fn read_bookmark(&self, id: &str) -> Option<BookmarkData> {
        let bookmarks_by_id = self.bookmarks_by_id.lock().unwrap();
        match bookmarks_by_id.get(id) {
            Some(bookmark) => Some(bookmark.clone()),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{adapters::clock::FakeClock, domain::data::DomainEventMeta, ports::Clock};

    #[test]
    fn test_read_model_exposes_bookmark_by_id() {
        let read_model = MemoryReadModel::new();
        let clock = FakeClock::new();

        read_model
            .update(&DomainEvent {
                meta: DomainEventMeta {
                    aggregate_id: "123".to_owned(),
                    created_at: clock.now(),
                },
                payload: DomainEventPayload::Bookmark(BookmarkEventPayload::Created {
                    url: "https://example.com".to_owned(),
                    title: "Example".to_owned(),
                }),
            })
            .unwrap();

        let bookmark = read_model.read_bookmark("123").unwrap();

        assert_eq!(bookmark.url, "https://example.com");
    }
}
