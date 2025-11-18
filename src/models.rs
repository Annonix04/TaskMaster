use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
pub enum Status {
    #[default]
    Pending,
    InProgress,
    Complete,
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct Task {
    pub title: String,
    pub status: Status,
}
