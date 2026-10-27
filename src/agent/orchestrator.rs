use crate::agent::planner::SimplePlanner;
use crate::agent::executor::Executor;
use crate::memory::MemoryStore;
use crate::task::TaskRegistry;
use crate::tools::ToolHandle;
use crate::utils::{Plan, ActionResult};
use anyhow::Result;
use std::sync::Arc;
use uuid::Uuid;

pub struct Orchestrator {
    memory: Arc<MemoryStore>,
    tool: Arc<dyn ToolHandle>,
    registry: Arc<TaskRegistry>,
}

impl Orchestrator {
    pub fn new(memory: Arc<MemoryStore>, tool: Arc<dyn ToolHandle>, registry: Arc<TaskRegistry>) -> Self {
        Self { memory, tool, registry }
    }

    pub async fn run_task(&self, id: &Uuid) -> Result<String> {
        let task = self.registry.get_task(&id).ok_or_else(|| anyhow::anyhow!("task not found"))?;
        let planner = SimplePlanner::new();
        let plan: Plan = planner.plan(&task.goal).await;

        // persist plan into memory
        self.memory.store_plan(id, &plan).await;

        let executor = Executor::new(self.tool.clone());
        let mut results: Vec<String> = Vec::new();

        for action in &plan.actions {
            let res: ActionResult = executor.execute(action).await;
            self.memory.append_event(id, &res.output).await;
            results.push(res.output.clone());
        }

        // Combine results into final result
        Ok(results.join("\n"))
    }
}
