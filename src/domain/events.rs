use serde::Serialize;

#[derive(std::fmt::Debug, PartialEq, Eq, Clone, Serialize)]
pub enum DomainEventPayload {
    BookmarkCreated {
        id: String,
        url: String,
        title: String,
    },
    BookmarkDeleted {
        id: String,
    },
    BookmarkTitleUpdated {
        id: String,
        title: String,
    },
}
