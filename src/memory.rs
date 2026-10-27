use crate::utils::{Plan, ActionResult};
use std::collections::HashMap;
use tokio::sync::RwLock;
use uuid::Uuid;

/// Simple in-memory store for plan and event traces.
#[derive(Debug, Default)]
pub struct MemoryStore {
    inner: RwLock<HashMap<Uuid, MemoryEntry>>,
}

#[derive(Debug, Clone)]
struct MemoryEntry {
    plan: Option<Plan>,
    events: Vec<String>,
}

impl MemoryStore {
    pub fn new() -> Self {
        Self { inner: RwLock::new(HashMap::new()) }
    }

    pub async fn store_plan(&self, id: &Uuid, plan: &Plan) {
        let mut w = self.inner.write().await;
        let e = w.entry(*id).or_insert_with(|| MemoryEntry { plan: None, events: Vec::new() });
        e.plan = Some(plan.clone());
    }

    pub async fn append_event(&self, id: &Uuid, event: &str) {
        let mut w = self.inner.write().await;
        let e = w.entry(*id).or_insert_with(|| MemoryEntry { plan: None, events: Vec::new() });
        e.events.push(event.to_string());
    }

    pub async fn get_events(&self, id: &Uuid) -> Option<Vec<String>> {
        let r = self.inner.read().await;
        r.get(id).map(|e| e.events.clone())
    }
}
