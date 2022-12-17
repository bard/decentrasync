use rouille::{input::json_input, router, start_server, Response};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::app;

#[derive(Deserialize)]
struct CreateRequest {
    url: String,
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

pub fn run(url: &str, store: Arc<dyn app::EventStore>) {
    start_server(url, move |req| -> Response {
        router!(
            req,
            (POST) (/bookmarks) => {
                match json_input::<CreateRequest>(req) {
                    Ok(data) => {
                        let id = app::create_bookmark(app::BookmarkInput {
                            url: data.url,
                            title: data.title
                        }, store.clone()).unwrap();
                        Response::json(&CreateResponse { id: id.to_owned()})
                    },
                    _ => Response::empty_400()
                }
            },
            (GET) (/bookmarks/{id: String}) => {
                match app::get_bookmark(app::BookmarkQuery { id },
                                        store.clone()) {
                    Some(bookmark) => Response::json(&ReadResponse{
                        id: bookmark.id,
                        url: bookmark.url,
                        title: bookmark.title
                    }),
                    _ => Response::empty_404()
                }
            },
            _ => {
                Response::empty_404()
            }
        )
    })
}
