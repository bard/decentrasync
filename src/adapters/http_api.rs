use rouille::{input::json_input, router, start_server, Response};
use rust_embed::RustEmbed;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use crate::app;

#[derive(RustEmbed)]
#[folder = "assets/"]
struct Asset;

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

pub fn run(url: &str, event_store: Arc<dyn app::EventStore>, read_model: Arc<dyn app::ReadModel>) {
    start_server(url, move |req| -> Response {
        router!(
            req,
            (GET) (/api/bookmarks/{id: String}) => {
                match app::query::read_bookmark(app::BookmarkQuery { id }, read_model.clone()) {
                    Some(bookmark) => Response::json(&ReadResponse{
                        id: bookmark.id,
                        url: bookmark.url,
                        title: bookmark.title
                    }),
                    _ => Response::empty_404()
                }
            },
            (GET) (/api/bookmarks)=> {
                match app::query::read_bookmarks(read_model.clone()) {
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
            (POST) (/api/bookmarks) => {
                match json_input::<CreateRequest>(req) {
                    Ok(data) => {
                        let id = Uuid::new_v4().to_string();

                        app::command::create_bookmark(app::Bookmark {
                            id: id.clone(),
                            url: data.url,
                            title: data.title
                        }, event_store.clone(), read_model.clone()).unwrap();

                        Response::json(&CreateResponse { id })
                    },
                    _ => Response::empty_400()
                }
            },
            (PUT) (/api/bookmarks/{id:  String}/title) => {
                match json_input::<UpdateTitleRequest>(req) {
                    Ok(data) => {
                        app::command::update_bookmark_title(id, data.title, event_store.clone(), read_model.clone()).unwrap();
                        Response::empty_204()
                    },
                    _ => Response::empty_400()
                }
            },
            (GET) (/) => {
                let asset = Asset::get("index.html").unwrap();
                Response::from_data("text/html", asset.data.into_owned())
            },
            _ => {
                Response::empty_404()
            }
        )
    })
}
