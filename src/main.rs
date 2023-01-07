use std::sync::Arc;

use clap::Parser;
use decentrasync::adapters::{
    http_api, memory_event_store::MemoryEventStore, memory_read_model::MemoryReadModel,
};

#[derive(Parser, Debug)]
#[command(about)]
struct Args {
    #[arg(short, long, default_value_t = 9111)]
    port: u16,
}

fn main() {
    let args = Args::parse();

    http_api::run(
        format!("localhost:{}", args.port).as_str(),
        Arc::new(MemoryEventStore::new()),
        Arc::new(MemoryReadModel::new()),
    );
}
