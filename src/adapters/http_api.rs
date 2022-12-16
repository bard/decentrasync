use rouille::router;
use serde::Deserialize;
use std::sync::Arc;

use crate::app;

#[derive(Deserialize)]
struct CreateRequest {
    url: String,
    title: String,
}

pub fn run(store: Arc<dyn app::EventStore>) {
    rouille::start_server("localhost:9111", move |req| -> rouille::Response {
        router!(req,
                (POST) (/bookmarks) => {
                    match rouille::input::json_input::<CreateRequest>(req) {
                        Ok(data) => {
                            let bookmark_id = app::create_bookmark(app::BookmarkInput {
                                url: data.url.clone(),
                                title: data.title.clone()
                            }, store.clone()).unwrap();
                            rouille::Response::text(bookmark_id)
                        },
                        _ => rouille::Response::text("error")
                    }
                },
                (GET) (/bookmarks/{id: String}) => {
                    match app::get_bookmark(app::BookmarkQuery {id: id.clone()}, store.clone()) {
                        Some(bookmark) => rouille::Response::text(format!("{:?}", bookmark)),
                        _ => rouille::Response::text("not found")
                    }
                },
                _ => {
                    rouille::Response::text("not found")
                }
        )
    })
}
