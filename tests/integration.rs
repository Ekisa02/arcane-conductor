use actix_web::{test, App};
use arcane_conductor::api::{create_app, AppState};
use arcane_conductor::memory::MemoryStore;
use arcane_conductor::task::TaskRegistry;
use arcane_conductor::tools::MockSearchTool;
use std::sync::Arc;
use serde_json::json;

#[actix_rt::test]
async fn test_run_and_status() {
    let memory = MemoryStore::new();
    let registry = TaskRegistry::new();
    let tool = Arc::new(MockSearchTool::default());
    let state = AppState { memory: Arc::new(memory), registry: Arc::new(registry), default_tool: tool };

    let app = test::init_service(create_app(state)).await;
    let req = test::TestRequest::post()
        .uri("/agent/run")
        .set_json(&json!({"goal":"Search for Rust async best practices."}))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
    let body: serde_json::Value = test::read_body_json(resp).await;
    assert!(body.get("task_id").is_some());
}
