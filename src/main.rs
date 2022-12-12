use std::sync::Arc;

use bookmarks::adapters::{http_api, memory_repository::MemoryRepository};

fn main() {
    let repo = Arc::new(MemoryRepository::new());
    http_api::run(repo);
}
