use crate::{
    app,
    data::{Bookmark, BookmarkQuery},
    ports,
};
use axum::{
    body::{self, Full},
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    response::Response,
    routing::{delete, get, post, put},
    Json, Router,
};
use hyper::header;
use rust_embed::RustEmbed;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

#[derive(RustEmbed)]
#[folder = "assets/"]
struct Asset;

struct ServiceDependencies {
    clock: Arc<dyn ports::Clock>,
    event_store: Arc<dyn ports::EventStore>,
    read_model: Arc<dyn ports::ReadModel>,
}

pub fn create_router(
    event_store: Arc<dyn ports::EventStore>,
    read_model: Arc<dyn ports::ReadModel>,
    clock: Arc<dyn ports::Clock>,
) -> Router {
    let deps = Arc::new(ServiceDependencies {
        event_store: event_store.clone(),
        read_model: read_model.clone(),
        clock: clock.clone(),
    });

    Router::new()
        .route("/", get(root))
        .route("/api/bookmarks", get(read_bookmarks))
        .route("/api/bookmarks", post(create_bookmark))
        .route("/api/bookmarks/:id", get(read_bookmark))
        .route("/api/bookmarks/:id", delete(delete_bookmark))
        .route("/api/bookmarks/:id/title", put(update_bookmark_title))
        .with_state(deps)
}

async fn root(State(_state): State<Arc<ServiceDependencies>>) -> impl IntoResponse {
    if let Some(asset) = Asset::get("index.html") {
        Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, "text/html")
            .body(body::boxed(Full::from(asset.data)))
            .unwrap()
    } else {
        (StatusCode::NOT_FOUND).into_response()
    }
}

#[derive(Serialize)]
struct ReadBookmarksResponse {
    bookmarks: Vec<ReadBookmarksResponseBookmarkEntry>,
}
#[derive(Serialize)]
struct ReadBookmarksResponseBookmarkEntry {
    id: String,
    url: String,
    title: String,
}

async fn read_bookmarks(State(state): State<Arc<ServiceDependencies>>) -> impl IntoResponse {
    match app::query::read_bookmarks(state.read_model.clone()) {
        Some(bookmarks) => (
            StatusCode::OK,
            Json(ReadBookmarksResponse {
                bookmarks: bookmarks
                    .iter()
                    .map(|b| ReadBookmarksResponseBookmarkEntry {
                        id: b.id.clone(),
                        url: b.url.clone(),
                        title: b.title.clone(),
                    })
                    .collect(),
            }),
        )
            .into_response(),
        _ => (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
    }
}

#[derive(Deserialize)]
struct CreateBookmarkRequestPayload {
    url: String,
    title: String,
}

#[derive(Serialize)]
struct CreateBoomarkResponsePayload {
    id: String,
}

async fn create_bookmark(
    State(state): State<Arc<ServiceDependencies>>,
    Json(payload): Json<CreateBookmarkRequestPayload>,
) -> impl IntoResponse {
    let id = Uuid::new_v4().to_string();

    match app::command::create_bookmark(
        Bookmark {
            id: id.clone(),
            url: payload.url,
            title: payload.title,
        },
        state.event_store.clone(),
        state.read_model.clone(),
        state.clock.clone(),
    ) {
        Ok(()) => (
            StatusCode::CREATED,
            Json(CreateBoomarkResponsePayload { id }),
        )
            .into_response(),
        _ => (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
    }
}

#[derive(Deserialize)]
struct UpdateBookmarkTitleRequestPayload {
    title: String,
}

async fn update_bookmark_title(
    State(state): State<Arc<ServiceDependencies>>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateBookmarkTitleRequestPayload>,
) -> impl IntoResponse {
    match app::command::update_bookmark_title(
        id,
        payload.title,
        state.event_store.clone(),
        state.read_model.clone(),
        state.clock.clone(),
    ) {
        Ok(()) => (StatusCode::NO_CONTENT, ()).into_response(),
        _ => (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
    }
}

async fn delete_bookmark(
    State(state): State<Arc<ServiceDependencies>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    match app::command::delete_bookmark(
        id,
        state.event_store.clone(),
        state.read_model.clone(),
        state.clock.clone(),
    ) {
        Ok(()) => (StatusCode::OK, ()).into_response(),
        _ => (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
    }
}

#[derive(Serialize)]
struct ReadBookmarkResponsePayload {
    id: String,
    url: String,
    title: String,
}

async fn read_bookmark(
    State(state): State<Arc<ServiceDependencies>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    match app::query::read_bookmark(BookmarkQuery { id }, state.read_model.clone()) {
        Some(bookmark) => (
            StatusCode::OK,
            Json(ReadBookmarkResponsePayload {
                id: bookmark.id,
                url: bookmark.url,
                title: bookmark.title,
            }),
        )
            .into_response(),
        _ => (StatusCode::NOT_FOUND, ()).into_response(),
    }
}
