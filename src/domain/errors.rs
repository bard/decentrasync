#[derive(thiserror::Error, Debug, PartialEq)]
pub enum DomainError {
    #[error("No such bookmark")]
    NoSuchBookmark,
    #[error("Bookmark already exists")]
    BookmarkAlreadyExists,
    #[error("Error interfacing with external system")]
    PortError,
}
