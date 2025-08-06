use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Task {
    pub id: usize,
    pub text: String,
    pub done: bool,
    pub created_at: DateTime<Local>,
    pub modified_at: DateTime<Local>,
}
