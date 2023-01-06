use rouille::{input::json_input, router, start_server, Response};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use crate::app;

#[derive(Deserialize)]
struct CreateRequest {
    url: String,
    title: String,
}

#[derive(Deserialize)]
struct UpdateTitleRequest {
    title: String,
}

#[derive(Serialize)]
struct CreateResponse {
    id: String,
}

#[derive(Serialize)]
struct ReadResponse {
    id: String,
    url: String,
    title: String,
}

#[derive(Serialize)]
struct ReadBookmarksResponseBookmarkEntry {
    id: String,
    url: String,
    title: String,
}

#[derive(Serialize)]
struct ReadBookmarksResponse {
    bookmarks: Vec<ReadBookmarksResponseBookmarkEntry>,
}

pub fn run(url: &str, store: Arc<dyn app::EventStore>) {
    start_server(url, move |req| -> Response {
        router!(
            req,
            (GET) (/bookmarks/{id: String}) => {
                match app::query::read_bookmark(app::BookmarkQuery { id },store.clone()) {
                    Some(bookmark) => Response::json(&ReadResponse{
                        id: bookmark.id,
                        url: bookmark.url,
                        title: bookmark.title
                    }),
                    _ => Response::empty_404()
                }
            },
            (GET) (/bookmarks)=> {
                match app::query::read_bookmarks(store.clone()) {
                    Some(bookmarks) => Response::json(&ReadBookmarksResponse{
                        bookmarks: bookmarks.iter().map(|b| ReadBookmarksResponseBookmarkEntry {
                            id: b.id.clone(),
                            url: b.url.clone(),
                            title: b.title.clone()
                        }).collect()
                    }),
                    _ => Response::empty_400()
                }
            },
            (POST) (/bookmarks) => {
                match json_input::<CreateRequest>(req) {
                    Ok(data) => {
                        let id = Uuid::new_v4().to_string();

                        app::command::create_bookmark(app::Bookmark {
                            id: id.clone(),
                            url: data.url,
                            title: data.title
                        }, store.clone()).unwrap();

                        Response::json(&CreateResponse { id })
                    },
                    _ => Response::empty_400()
                }
            },
            (PUT) (/bookmarks/{id:  String}/title) => {
                match json_input::<UpdateTitleRequest>(req) {
                    Ok(data) => {
                        app::command::update_bookmark_title(id, data.title, store.clone()).unwrap();
                        Response::empty_204()
                    },
                    _ => Response::empty_400()
                }
            },
            _ => {
                Response::empty_404()
            }
        )
    })
}
