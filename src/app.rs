use std::sync::Arc;

type BookmarkId = String;

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
    fn get_all_events(&self) -> Vec<DomainEvent>;
}

pub fn get_bookmark(query: BookmarkQuery, store: Arc<dyn EventStore>) -> Option<Bookmark> {
    // TODO replace with read model
    store.get_all_events()
        .iter()
        .fold(None, |acc, event| match event {
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
        })
}

pub fn delete_bookmark(_bookmark_query: BookmarkQuery) -> () {}

pub fn create_bookmark(
    bookmark_input: BookmarkInput,
    store: Arc<dyn EventStore>,
) -> Result<String, ()> {
    let _log_res = store.push(DomainEvent::BookmarkCreated {
        id: bookmark_input.url.clone(), // XXX temporary
        url: bookmark_input.url.clone(),
        title: bookmark_input.title.clone(),
    });

    Ok(bookmark_input.url)
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
