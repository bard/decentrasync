use std::sync::Arc;

use decentralized_bookmarks::adapters::{http_api, memory_event_store::MemoryEventStore};

fn main() {
    let store = Arc::new(MemoryEventStore::new());
    http_api::run(store);
}
