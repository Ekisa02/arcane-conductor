//! A minimal planner that converts a textual goal into a linear plan of actions.

use crate::utils::{Action, Plan};

pub struct SimplePlanner;

impl SimplePlanner {
    pub fn new() -> Self {
        SimplePlanner {}
    }

    pub async fn plan(&self, goal: &str) -> Plan {
        // Very small heuristic: split on sentence boundaries and create actions.
        let mut actions = Vec::new();
        for (i, part) in goal.split('.').filter(|s| !s.trim().is_empty()).enumerate() {
            let desc = part.trim().to_string();
            // If it looks like a search request, mark a tool
            let action = if desc.to_lowercase().contains("research") || desc.to_lowercase().contains("search") {
                Action::tool(format!("search: {}", desc))
            } else {
                Action::simple(desc)
            };
            actions.push(action);
        }
        Plan { actions }
    }
}
