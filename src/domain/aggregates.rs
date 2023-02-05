use super::{
    commands::BookmarkCommand,
    data::{Aggregate, DomainEventMeta},
    errors::DomainError,
    events::BookmarkEventPayload,
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
    type EventPayload = BookmarkEventPayload;

    fn handle_command(&self, command: &Self::Command) -> Result<Self::EventPayload, DomainError> {
        match command {
            BookmarkCommand::BookmarkPage { url, title } => match self.state {
                State::Deleted => Err(DomainError::NoSuchBookmark),
                State::Created => Err(DomainError::BookmarkAlreadyExists),
                State::Nonexistent => Ok(BookmarkEventPayload::Created {
                    url: url.clone(),
                    title: title.clone(),
                }),
            },
            BookmarkCommand::Delete => match self.state {
                State::Deleted => Err(DomainError::NoSuchBookmark),
                State::Nonexistent => Err(DomainError::NoSuchBookmark),
                State::Created => Ok(BookmarkEventPayload::Deleted),
            },
            BookmarkCommand::UpdateTitle { title } => match self.state {
                State::Deleted => Err(DomainError::NoSuchBookmark),
                State::Nonexistent => Err(DomainError::NoSuchBookmark),
                State::Created => Ok(BookmarkEventPayload::TitleUpdated {
                    title: title.clone(),
                }),
            },
        }
    }

    fn apply_event(
        mut self,
        payload: &BookmarkEventPayload,
        meta: &DomainEventMeta,
    ) -> BookmarkAggregate {
        match &payload {
            BookmarkEventPayload::Created { url, title } => {
                if *meta.aggregate_id == self.id {
                    self.state = State::Created;
                    self.title = title.clone();
                    self.url = url.clone();
                }
            }
            BookmarkEventPayload::Deleted => {
                if *meta.aggregate_id == self.id {
                    self.state = State::Deleted;
                }
            }
            BookmarkEventPayload::TitleUpdated { title } => {
                if *meta.aggregate_id == self.id {
                    self.title = title.clone();
                }
            }
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{adapters::clock::FakeClock, domain::data::DomainEventMeta, ports::Clock};

    #[test]
    fn test_deleted_bookmark_cannot_be_acted_upon() {
        let clock = FakeClock::new();
        let bookmark = BookmarkAggregate::new("123456".to_owned());
        let bookmark = bookmark.apply_event(
            &BookmarkEventPayload::Created {
                url: "https://example.com".to_owned(),
                title: "Example".to_owned(),
            },
            &DomainEventMeta {
                aggregate_id: "123456".to_owned(),
                created_at: clock.now(),
            },
        );
        let bookmark = bookmark.apply_event(
            &BookmarkEventPayload::Deleted,
            &DomainEventMeta {
                aggregate_id: "123456".to_owned(),
                created_at: clock.now(),
            },
        );

        let err = bookmark
            .handle_command(&BookmarkCommand::UpdateTitle {
                title: "Foobar".to_owned(),
            })
            .unwrap_err();

        assert_eq!(err, DomainError::NoSuchBookmark);
    }

    #[test]
    fn test_bookmark_with_duplicate_id_cannot_be_created() {
        let clock = FakeClock::new();
        let bookmark = BookmarkAggregate::new("123456".to_owned());
        let bookmark = bookmark.apply_event(
            &BookmarkEventPayload::Created {
                url: "https://example.com".to_owned(),
                title: "Example".to_owned(),
            },
            &DomainEventMeta {
                aggregate_id: "123456".to_owned(),
                created_at: clock.now(),
            },
        );

        let err = bookmark
            .handle_command(&BookmarkCommand::BookmarkPage {
                url: "https://foobar.com".to_owned(),
                title: "Foobar".to_owned(),
            })
            .unwrap_err();

        assert_eq!(err, DomainError::BookmarkAlreadyExists)
    }

    #[test]
    fn test_bookmarking_page_generates_create_event() {
        let bookmark = BookmarkAggregate::new("123456".to_owned());

        let event_payload = bookmark
            .handle_command(&BookmarkCommand::BookmarkPage {
                url: "https://example.com".to_owned(),
                title: "Example".to_owned(),
            })
            .unwrap();

        assert_eq!(
            event_payload,
            BookmarkEventPayload::Created {
                url: "https://example.com".to_owned(),
                title: "Example".to_owned(),
            }
        )
    }
}
