use actix_web::{HttpServer};
use std::sync::Arc;
use log::info;

use arcane_conductor::api::{create_app, AppState};
use arcane_conductor::memory::MemoryStore;
use arcane_conductor::tools::MockSearchTool;
use arcane_conductor::task::TaskRegistry;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    info!("Starting Arcane Conductor server...");

    let memory = MemoryStore::new();
    let registry = TaskRegistry::new();
    let tool = Arc::new(MockSearchTool::default());

    let state = AppState {
        memory: Arc::new(memory),
        registry: Arc::new(registry),
        default_tool: tool,
    };

    HttpServer::new(move || create_app(state.clone()))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
