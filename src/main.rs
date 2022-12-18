use std::sync::Arc;

use dumb_channel_decentralized_sync::adapters::{http_api, memory_event_store::MemoryEventStore};

fn main() {
    http_api::run("localhost:9111", Arc::new(MemoryEventStore::new()));
}
