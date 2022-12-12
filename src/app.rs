use crate::entities;
use crate::repositories::Repository;
use crate::values::{BookmarkInput, BookmarkQuery};
use std::sync::Arc;

pub fn read_bookmark(
    bookmark_query: BookmarkQuery,
    repo: Arc<dyn Repository>,
) -> Result<entities::Bookmark, ()> {
    repo.fetch_first(bookmark_query)
}

pub fn delete_bookmark(_bookmark_query: BookmarkQuery) -> () {}

pub fn create_bookmark(
    bookmark_input: BookmarkInput,
    repo: Arc<dyn Repository>,
) -> Result<String, ()> {
    let _res = repo.insert(entities::Bookmark {
        id: bookmark_input.url.clone(), // XXX temporary
        url: bookmark_input.url.clone(),
        title: bookmark_input.title,
    });

    Ok(bookmark_input.url)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repositories::InMemoryRepository;

    #[test]
    fn test_created_bookmark_is_saved_in_repository() {
        let repo = Arc::new(InMemoryRepository::new());
        let id = create_bookmark(
            BookmarkInput {
                url: "http://bar".to_string(),
                title: "bar".to_string(),
            },
            repo.clone(),
        )
        .unwrap();

        let bookmark = read_bookmark(BookmarkQuery { id: id.clone() }, repo.clone()).unwrap();

        assert_eq!(
            bookmark,
            entities::Bookmark {
                id,
                url: "http://bar".to_string(),
                title: "bar".to_string(),
            }
        )
    }
}
