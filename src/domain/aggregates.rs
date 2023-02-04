use crate::data::{Aggregate, DomainError, DomainEvent};

use super::{
    commands::BookmarkCommand,
    events::{BookmarkEventPayload, DomainEventPayload},
};

enum State {
    Nonexistent,
    Created,
    Deleted,
}

pub struct BookmarkAggregate {
    pub id: String,
    pub title: String,
    pub url: String,
    state: State,
}

impl BookmarkAggregate {
    pub fn new(id: String) -> Self {
        Self {
            id,
            state: State::Nonexistent,
            title: "".to_string(),
            url: "".to_string(),
        }
    }
}

impl Aggregate for BookmarkAggregate {
    type Command = BookmarkCommand;

    fn handle_command(&self, command: &BookmarkCommand) -> Result<DomainEventPayload, DomainError> {
        match command {
            BookmarkCommand::BookmarkPage { url, title } => match self.state {
                State::Deleted => Err(DomainError::NoSuchBookmark),
                State::Created => Err(DomainError::BookmarkAlreadyExists),
                State::Nonexistent => Ok(DomainEventPayload::Bookmark(
                    BookmarkEventPayload::Created {
                        id: self.id.clone(),
                        url: url.clone(),
                        title: title.clone(),
                    },
                )),
            },
            BookmarkCommand::Delete => match self.state {
                State::Deleted => Err(DomainError::NoSuchBookmark),
                State::Nonexistent => Err(DomainError::NoSuchBookmark),
                State::Created => Ok(DomainEventPayload::Bookmark(
                    BookmarkEventPayload::Deleted {
                        id: self.id.clone(),
                    },
                )),
            },
            BookmarkCommand::UpdateTitle { title } => match self.state {
                State::Deleted => Err(DomainError::NoSuchBookmark),
                State::Nonexistent => Err(DomainError::NoSuchBookmark),
                State::Created => Ok(DomainEventPayload::Bookmark(
                    BookmarkEventPayload::TitleUpdated {
                        id: self.id.clone(),
                        title: title.clone(),
                    },
                )),
            },
        }
    }

    fn apply_event(&mut self, event: &DomainEvent) {
        match &event.payload {
            DomainEventPayload::Bookmark(BookmarkEventPayload::Created {
                id,
                url,
                title,
            }) => {
                if *id == self.id {
                    self.state = State::Created;
                    self.title = title.clone();
                    self.url = url.clone();
                }
            }
            DomainEventPayload::Bookmark(BookmarkEventPayload::Deleted {
                id,
            }) => {
                if *id == self.id {
                    self.state = State::Deleted;
                }
            }
            DomainEventPayload::Bookmark(
                BookmarkEventPayload::TitleUpdated { id, title },
            ) => {
                if *id == self.id {
                    self.title = title.clone();
                }
            }
            _ => (),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{adapters::clock::FakeClock, data::DomainEventMeta, ports::Clock};

    #[test]
    fn test_deleted_bookmark_cannot_be_acted_upon() {
        let mut bookmark = BookmarkAggregate::new("123456".to_owned());
        let clock = FakeClock::new();
        bookmark.apply_event(&DomainEvent {
            meta: DomainEventMeta {
                created_at: clock.now(),
            },
            payload: DomainEventPayload::Bookmark(
                BookmarkEventPayload::Created {
                    id: "123456".to_owned(),
                    url: "https://example.com".to_owned(),
                    title: "Example".to_owned(),
                },
            ),
        });
        bookmark.apply_event(&DomainEvent {
            meta: DomainEventMeta {
                created_at: clock.now(),
            },
            payload: DomainEventPayload::Bookmark(
                BookmarkEventPayload::Deleted {
                    id: "123456".to_owned(),
                },
            ),
        });

        let err = bookmark
            .handle_command(&BookmarkCommand::UpdateTitle {
                title: "Foobar".to_owned(),
            })
            .unwrap_err();

        assert_eq!(err, DomainError::NoSuchBookmark);
    }

    #[test]
    fn test_bookmark_with_duplicate_id_cannot_be_created() {
        let mut bookmark = BookmarkAggregate::new("123456".to_owned());
        let clock = FakeClock::new();
        bookmark.apply_event(&DomainEvent {
            meta: DomainEventMeta {
                created_at: clock.now(),
            },
            payload: DomainEventPayload::Bookmark(
                BookmarkEventPayload::Created {
                    id: "123456".to_owned(),
                    url: "https://example.com".to_owned(),
                    title: "Example".to_owned(),
                },
            ),
        });

        let err = bookmark
            .handle_command(&BookmarkCommand::BookmarkPage {
                url: "https://foobar.com".to_owned(),
                title: "Foobar".to_owned(),
            })
            .unwrap_err();

        assert_eq!(err, DomainError::BookmarkAlreadyExists)
    }
}
