use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Plan {
    pub actions: Vec<Action>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Action {
    Simple { id: Uuid, description: String },
    Tool { id: Uuid, description: String },
}

impl Action {
    pub fn simple(desc: String) -> Self {
        Action::Simple { id: Uuid::new_v4(), description: desc }
    }

    pub fn tool(desc: String) -> Self {
        Action::Tool { id: Uuid::new_v4(), description: desc }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ActionResult {
    pub success: bool,
    pub output: String,
}

impl ActionResult {
    pub fn ok(output: String) -> Self {
        ActionResult { success: true, output }
    }

    pub fn err(output: String) -> Self {
        ActionResult { success: false, output }
    }
}
