use std::sync::Arc;

use bookmarks::adapters::{http_api, memory_event_log::MemoryEventLog};

fn main() {
    let log = Arc::new(MemoryEventLog::new());
    http_api::run(log);
}
