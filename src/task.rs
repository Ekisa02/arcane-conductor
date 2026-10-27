use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::RwLock;
use uuid::Uuid;
use std::sync::Arc;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TaskStatus {
    Pending,
    Running,
    Completed,
    Failed(String),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Task {
    pub id: Uuid,
    pub goal: String,
    pub status: TaskStatus,
    pub result: Option<String>,
}

impl Task {
    pub fn new(goal: String) -> Self {
        Task { id: Uuid::new_v4(), goal, status: TaskStatus::Pending, result: None }
    }
}

#[derive(Clone)]
pub struct TaskRegistry {
    inner: Arc<RwLock<HashMap<Uuid, Task>>>,
}

impl TaskRegistry {
    pub fn new() -> Self {
        Self { inner: Arc::new(RwLock::new(HashMap::new())) }
    }

    pub fn create_task(&self, goal: String) -> Uuid {
        let t = Task::new(goal);
        let id = t.id;
        let mut map = futures::executor::block_on(self.inner.write());
        map.insert(id, t);
        id
    }

    pub fn get_task(&self, id: &Uuid) -> Option<Task> {
        let map = futures::executor::block_on(self.inner.read());
        map.get(id).cloned()
    }

    pub fn update_status(&self, id: &Uuid, status: TaskStatus) -> Option<()> {
        let mut map = futures::executor::block_on(self.inner.write());
        if let Some(t) = map.get_mut(id) {
            t.status = status;
            Some(())
        } else {
            None
        }
    }

    pub fn complete_task(&self, id: &Uuid, result: Option<String>) -> Option<()> {
        let mut map = futures::executor::block_on(self.inner.write());
        if let Some(t) = map.get_mut(id) {
            t.status = TaskStatus::Completed;
            t.result = result;
            Some(())
        } else {
            None
        }
    }

    pub fn fail_task(&self, id: &Uuid, reason: String) -> Option<()> {
        let mut map = futures::executor::block_on(self.inner.write());
        if let Some(t) = map.get_mut(id) {
            t.status = TaskStatus::Failed(reason);
            Some(())
        } else {
            None
        }
    }
}
