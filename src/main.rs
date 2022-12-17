use std::sync::Arc;

use decentralized_bookmarks::adapters::{http_api, memory_event_store::MemoryEventStore};

fn main() {
    http_api::run("localhost:9111", Arc::new(MemoryEventStore::new()));
}
