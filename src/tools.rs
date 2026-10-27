use async_trait::async_trait;
use std::sync::Arc;

/// ToolHandle provides a simple interface for executor to call external tools.
#[async_trait]
pub trait ToolHandle: Send + Sync {
    async fn call(&self, input: &str) -> String;
}

#[derive(Default)]
pub struct MockSearchTool {}

#[async_trait]
impl ToolHandle for MockSearchTool {
    async fn call(&self, input: &str) -> String {
        // A mock search that echoes the query with a pretend result.
        format!("[mock-search-results for '{}'] - summary: Example findings about {}.", input, input)
    }
}

// expose a boxed trait object type alias
pub type ToolHandleObject = Arc<dyn ToolHandle>;
