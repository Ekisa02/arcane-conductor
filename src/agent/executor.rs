use crate::tools::ToolHandle;
use crate::utils::{Action, ActionResult};

pub struct Executor {
    pub tool: std::sync::Arc<dyn ToolHandle>,
}

impl Executor {
    pub fn new(tool: std::sync::Arc<dyn ToolHandle>) -> Self {
        Self { tool }
    }

    pub async fn execute(&self, action: &Action) -> ActionResult {
        match action {
            Action::Simple { id: _, description } => {
                // simple local action
                ActionResult::ok(format!("Executed: {}", description))
            }
            Action::Tool { id: _, description } => {
                // description prefixed with tool:, e.g. "search: topic"
                let out = self.tool.call(description.as_str()).await;
                ActionResult::ok(out)
            }
        }
    }
}
