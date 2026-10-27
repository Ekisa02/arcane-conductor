use actix_web::{web, App, HttpResponse, HttpServer, Responder, HttpRequest, HttpMessage};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use crate::memory::MemoryStore;
use crate::task::{TaskRegistry, TaskStatus};
use crate::tools::ToolHandle;
use crate::agent::orchestrator::Orchestrator;

#[derive(Clone)]
pub struct AppState {
    pub memory: Arc<MemoryStore>,
    pub registry: Arc<TaskRegistry>,
    pub default_tool: Arc<dyn ToolHandle>,
}

#[derive(Deserialize)]
struct RunRequest {
    goal: String,
}

#[derive(Serialize)]
struct RunResponse {
    task_id: String,
}

pub fn create_app(state: AppState) -> App<impl actix_web::dev::ServiceFactory> {
    App::new()
        .app_data(web::Data::new(state))
        .route("/agent/run", web::post().to(run_agent))
        .route("/agent/status/{id}", web::get().to(get_status))
}

async fn run_agent(state: web::Data<AppState>, payload: web::Json<RunRequest>) -> impl Responder {
    let goal = payload.goal.clone();
    let task_id = state.registry.create_task(goal.clone());

    // Spawn a background orchestration
    let registry = state.registry.clone();
    let memory = state.memory.clone();
    let tool = state.default_tool.clone();
    let id = task_id.clone();

    // Using tokio to spawn background processing
    let orchestrator = Orchestrator::new(memory, tool.clone(), registry.clone());
    tokio::spawn(async move {
        let _ = registry.update_status(&id, TaskStatus::Running);
        let res = orchestrator.run_task(&id).await;
        match res {
            Ok(output) => {
                let _ = registry.complete_task(&id, Some(output));
            }
            Err(e) => {
                let _ = registry.fail_task(&id, format!("{}", e));
            }
        }
    });

    HttpResponse::Ok().json(RunResponse { task_id })
}

async fn get_status(state: web::Data<AppState>, req: HttpRequest) -> impl Responder {
    let id_str = req.match_info().get("id").unwrap_or("");
    match Uuid::parse_str(id_str) {
        Ok(id) => match state.registry.get_task(&id) {
            Some(t) => HttpResponse::Ok().json(t),
            None => HttpResponse::NotFound().body("task not found"),
        },
        Err(_) => HttpResponse::BadRequest().body("invalid uuid"),
    }
}
