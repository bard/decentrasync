use std::sync::Arc;

use clap::Parser;
use decentrasync::adapters::{
    clock::SystemClock, http_api, memory_event_store::MemoryEventStore,
    memory_read_model::MemoryReadModel,
};

#[derive(Parser, Debug)]
#[command(about)]
struct Args {
    #[arg(short, long, default_value_t = 9111)]
    port: u16,
}

fn main() {
    let args = Args::parse();

    let event_store = Arc::new(MemoryEventStore::new());
    let read_model = Arc::new(MemoryReadModel::new());
    let clock = Arc::new(SystemClock::new());

    http_api::run(
        format!("localhost:{}", args.port).as_str(),
        event_store.clone(),
        read_model.clone(),
        clock.clone(),
    );
}
