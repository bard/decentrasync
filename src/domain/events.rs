use serde::Serialize;

#[derive(std::fmt::Debug, PartialEq, Eq, Clone, Serialize)]
#[serde(tag = "type")]
pub enum BookmarkEventPayload {
    Created {
        id: String,
        url: String,
        title: String,
    },
    Deleted {
        id: String,
    },
    TitleUpdated {
        id: String,
        title: String,
    },
}

#[derive(std::fmt::Debug, PartialEq, Eq, Clone, Serialize)]
pub enum OtherEventPayload {
    SomethingHappened,
}

#[derive(std::fmt::Debug, PartialEq, Eq, Clone, Serialize)]
#[serde(tag = "aggregate")]
pub enum DomainEventPayload {
    Bookmark(BookmarkEventPayload),
    Other(OtherEventPayload),
}
