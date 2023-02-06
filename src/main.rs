use clap::Parser;
use decentrasync::{
    adapters::{
        clock::SystemClock, file_event_store::FileSystemEventStore, http_api_axum,
        memory_read_model::MemoryReadModel,
    },
    app,
};
use std::{env, net::SocketAddr, path::Path, sync::Arc};

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
    let log_folder_path = Path::new(&env::temp_dir()).join("decentrasync");

    let event_store = Arc::new(FileSystemEventStore::new(log_folder_path.as_os_str()));
    let read_model = Arc::new(MemoryReadModel::new());
    let clock = Arc::new(SystemClock::new());

    app::init(event_store.clone(), read_model.clone());

    axum::Server::bind(&addr)
        .serve(
            http_api_axum::create_router(event_store.clone(), read_model.clone(), clock.clone())
                .into_make_service(),
        )
        .await
        .unwrap();
}
