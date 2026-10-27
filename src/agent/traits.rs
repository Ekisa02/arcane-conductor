use async_trait::async_trait;

// Reserved place for cross-cutting agent traits (Planner/Executor as traits) if needed in the future.

#[async_trait]
pub trait Planner: Send + Sync {
    async fn plan(&self, goal: &str) -> anyhow::Result<crate::utils::Plan>;
}

#[async_trait]
pub trait Executor: Send + Sync {
    async fn execute(&self, action: &crate::utils::Action) -> crate::utils::ActionResult;
}
