mod adapters;
mod entities;
mod repositories;
mod values;

use std::sync::Arc;

fn main() {
    let repo = Arc::new(repositories::InMemoryRepository::new());
    adapters::http::run(repo);
}
