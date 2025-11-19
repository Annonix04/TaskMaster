use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
pub enum Status {
    #[default]
    Pending,
    InProgress,
    Complete,
}

#[derive(Serialize, Deserialize, Clone, Debug, Copy, PartialEq, Default)]
pub enum Themes {
    #[default]
    Default,
    KanagawaWave,
    Dark,
    Light,
    Nord,
    SolarizedDark,
    SolarizedLight,
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct Task {
    pub title: String,
    pub status: Status,
}

#[derive(Serialize, Deserialize)]
pub struct Tasks {
    pub list: Vec<Task>,
    #[serde(skip, default)]
    pub adding_after: Option<usize>,
    #[serde(skip, default)]
    pub new_title: String,
    pub themes: Vec<Themes>,
    pub selected_theme: Option<Themes>
}

#[derive(Debug, Clone)]
pub enum Message {
    Forward(usize),
    AddAfter(usize),
    UpdateNewTitle(String),
    ConfirmAdd,
    CancelAdd,
    Remove(usize),
    ThemeChanged(Themes),
}
