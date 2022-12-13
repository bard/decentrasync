use rouille::router;
use std::sync::Arc;

use crate::app::EventStore;

pub fn run(_store: Arc<dyn EventStore>) {
    rouille::start_server("localhost:9111", move |req| -> rouille::Response {
        router!(req,
                (GET) (/) => {
                    rouille::Response::text("hello")
                },
                _ => {
                    rouille::Response::text("not found")
                }
        )
    })
}
