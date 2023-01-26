use std::{net::SocketAddr, sync::Arc};

use clap::Parser;
use decentrasync::adapters::{
    clock::SystemClock, http_api_axum, memory_event_store::MemoryEventStore,
    memory_read_model::MemoryReadModel,
};

#[derive(Parser, Debug)]
#[command(about)]
struct Args {
    #[arg(short, long, default_value_t = 9111)]
    port: u16,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let addr = SocketAddr::from(([127, 0, 0, 1], args.port));

    axum::Server::bind(&addr)
        .serve(
            http_api_axum::create_router(
                Arc::new(MemoryEventStore::new()),
                Arc::new(MemoryReadModel::new()),
                Arc::new(SystemClock::new()),
            )
            .into_make_service(),
        )
        .await
        .unwrap();
}
