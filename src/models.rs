use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize, PartialEq, PartialOrd)]
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
    KanagawaDragon,
    KanagawaLotus,
    Nord,
    Ferra,
    Dracula,
    Dark,
    Light,
    SolarizedDark,
    SolarizedLight,
    GruvboxDark,
    GruvboxLight,
    Moonfly,
    Nightfly,
    Oxocarbon,
    TokyoNight,
    TokyoNightLight,
    TokyoNightStorm,
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct Task {
    pub title: String,
    pub status: Status,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Tasks {
    #[serde(default)]
    pub title: String,
    pub list: Vec<Task>,
    #[serde(skip, default)]
    pub adding_after: Option<usize>,
    #[serde(skip, default)]
    pub new_title: String,
    #[serde(skip, default)]
    pub editing: Option<usize>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct List {
    pub lists: Vec<Tasks>,
    #[serde(skip, default)]
    pub selected: Option<usize>,
    #[serde(skip, default)]
    pub adding_after: Option<usize>,
    #[serde(skip, default)]
    pub new_title: String,
    #[serde(skip, default)]
    pub editing: Option<usize>,
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
    ChangeTitle(usize),
    ConfirmEdit,
    CancelEdit,

    ThemeChanged(Themes),
    SelectList(usize),
    BackToLists,
    AddListAfter(usize),
    UpdateListTitle(String),
    ConfirmAddList,
    CancelAddList,
    RemoveList(usize),
    ChangeListTitle(usize),
    ConfirmListEdit,
    CancelListEdit,
}
