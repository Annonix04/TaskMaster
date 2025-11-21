#![allow(unused_imports, dead_code)]
use crate::models::*;
use iced::{
    self,
    Alignment,
    Element,
    Fill,
    widget::*,
    widget::{
        text::Wrapping,
        column,
        container,
    },
    Length,
    FillPortion,
};
use serde::{
    Deserialize, 
    Serialize,
};
use std::{
    fs,
    io::{
        self, 
        Read, 
        Write,
    },
    path::{
        Path, 
        PathBuf,
    },
};
use std::fmt::{
    Display,
    Formatter,
};

#[inline]
fn logs_path() -> Option<PathBuf> {
    if let Ok(home) = std::env::var("HOME") {
        return Some(Path::new(&home).join("Tasks").join("bin").join("logs.txt"));
    }
    if let Ok(userprofile) = std::env::var("USERPROFILE") {
        return Some(Path::new(&userprofile).join("Tasks").join("bin").join("logs.txt"));
    }
    None
}

#[cfg(debug_assertions)]
#[inline]
fn log_error(msg: &str) {
    eprintln!("{msg}");
}

#[cfg(not(debug_assertions))]
fn log_error(msg: &str) {
    if let Some(path) = logs_path() {
        if let Some(parent) = path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        let timestamp = chrono::Local::now()
            .format("%Y-%m-%d %H:%M:%S")
            .to_string();
        let line = format!("[{}] {}\n", timestamp, msg);
        let mut file = match std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&path)
        {
            Ok(f) => f,
            Err(_) => return,
        };
        let _ = file.write_all(line.as_bytes());
    }
}

impl Display for Themes {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Tasks {
    pub fn update(&mut self, msg: Message) {
        match msg {
            Message::AddAfter(index) => {
                self.adding_after = Some(index);
            }
            Message::UpdateNewTitle(title) => {
                self.new_title = title;
            }
            Message::ConfirmAdd => {
                let title = self.new_title.trim().to_string();
                if !title.is_empty() {
                    self.list.push(Task {
                        title,
                        status: Status::Pending,
                    });
                }
                self.new_title.clear();
                self.adding_after = None;
            }
            Message::CancelAdd => {
                self.new_title.clear();
                self.adding_after = None;
            }
            Message::Remove(index) => {
                if index < self.list.len() {
                    self.list.remove(index);
                }
            }
            Message::Forward(index) => {
                if let Some(task) = self.list.get_mut(index) {
                    task.update(Message::Forward(index));
                }
            }
            Message::ChangeTitle(index) => {
                if let Some(_) = self.list.get(index) {
                    self.editing = Some(index);
                }
            }
            Message::ConfirmEdit => {
                if let Some(index) = self.editing.take() {
                    if let Some(task) = self.list.get_mut(index) {
                        task.title = self.new_title.clone();
                    }
                    self.new_title.clear();
                }
            }
            Message::CancelEdit => {
                self.editing = None;
                self.new_title.clear();
            }
            _ => {}
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let mut interface = column![]
            .spacing(16)
            .padding(16);

        if self.list.is_empty() {
            if self.adding_after == Some(0) {
                interface = interface.push(
                    row![
                        text_input("New task title...", &self.new_title)
                            .on_input(Message::UpdateNewTitle)
                            .padding(8)
                            .width(Fill),
                        button("Save").on_press(Message::ConfirmAdd),
                        button("Cancel").on_press(Message::CancelAdd),
                    ]
                    .spacing(8),
                );
            } else {
                interface = interface.push(button("Add Task").on_press(Message::AddAfter(0)));
            }
        }

        for (i, task) in self.list.iter().enumerate() {
            if i > 0 {
                interface = interface.push(container(Rule::horizontal(1)).width(Fill));
            }

            interface = interface.push(container(task.view(i)).padding(8));

            if self.editing == Some(i) {
                interface = interface.push(
                    row![
                        text_input("New task title...", &self.new_title)
                            .on_input(Message::UpdateNewTitle)
                            .padding(8)
                            .width(Fill),
                        button("Save").on_press(Message::ConfirmEdit),
                        button("Cancel").on_press(Message::CancelEdit),
                    ]
                    .spacing(8)
                )
            }
        }

        if !self.list.is_empty() {
            let end_index = self.list.len();
            if self.adding_after == Some(end_index) {
                interface = interface.push(
                    row![
                        text_input("New task title...", &self.new_title)
                            .on_input(Message::UpdateNewTitle)
                            .padding(8)
                            .width(Fill),
                        button("Save").on_press(Message::ConfirmAdd),
                        button("Cancel").on_press(Message::CancelAdd),
                    ]
                    .spacing(8)
                    .padding(4),
                );
            } else {
                interface = interface.push(button("Add Task")
                    .style(button::secondary)
                    .on_press(Message::AddAfter(end_index)));
            }
        }
        let scrollable_list = scrollable(interface.spacing(12)).height(Fill);

        scrollable_list.into()
    }
}

impl Default for Tasks {
    fn default() -> Self {
        Self {
            title: String::from("Untitled"),
            list: Vec::new(),
            adding_after: None,
            new_title: String::new(),
            editing: None,
        }
    }
}

impl Task {
    fn update(&mut self, msg: Message) {
        match msg {
            Message::Forward(_) => {
                self.status = match &self.status {
                    Status::Pending => Status::InProgress,
                    Status::InProgress => Status::Complete,
                    Status::Complete => Status::InProgress,
                }
            }
            _ => {}
        }
    }

    fn view(&self, id: usize) -> Element<'_, Message> {
        let mut interface = row![
            text(&self.title).size(20).wrapping(Wrapping::Word).width(FillPortion(4)),
            text(format!(" - {:?}", self.status))
                .wrapping(Wrapping::None)
                .size(16)
                .style(text::success),
        ]
        .spacing(12)
        .align_y(Alignment::Center);

        interface = match self.status {
            Status::Pending => interface.push(
                container(
                    button("Start")
                        .on_press(Message::Forward(id))
                ).width(Fill)
            ),
            Status::InProgress | Status::Complete => {
                let checked = matches!(self.status, Status::Complete);
                interface.push(
                    checkbox("Complete", checked)
                        .on_toggle(move |_| Message::Forward(id))
                        .width(Fill)
                )
            }
        };

        interface = interface.push(
            button("Edit")
                .style(button::secondary)
                .on_press(Message::ChangeTitle(id))
        );

        interface = interface.push(
            button("Remove")
                .style(button::danger)
                .on_press(Message::Remove(id))
        );

        container(interface).padding(4).width(Fill).into()
    }
}

impl List {
    fn data_path() -> Option<PathBuf> {
        if let Ok(home) = std::env::var("HOME") {
            return Some(Path::new(&home).join("Tasks").join("lists.json"));
        }
        if let Ok(userprofile) = std::env::var("USERPROFILE") {
            return Some(Path::new(&userprofile).join("Tasks").join("lists.json"));
        }
        None
    }

    fn ensure_parent_dir(path: &Path) -> io::Result<()> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
        } else {
            Ok(())
        }
    }

    fn save(&self) {
        if let Some(path) = Self::data_path() {
            if let Err(e) = Self::ensure_parent_dir(&path) {
                log_error(&format!("Failed to create data directory: {e}"));
                return;
            }
            match serde_json::to_string_pretty(self) {
                Ok(json) => {
                    if let Err(e) = fs::write(&path, json) {
                        log_error(&format!(
                            "Failed to write lists to {}: {e}",
                            path.display()
                        ));
                    }
                }
                Err(e) => log_error(&format!("Failed to serialize lists: {e}")),
            }
        } else {
            log_error("Could not resolve home directory to save lists.");
        }
    }

    fn new() -> Self {
        Self {
            lists: Vec::new(),
            themes: Vec::new(),
            selected: None,
            adding_after: None,
            new_title: String::new(),
            editing: None,
            selected_theme: None,
        }
    }

    fn new_with_themes(themes: Vec<Themes>) -> Self {
        Self {
            themes,
            lists: Vec::new(),
            selected: None,
            adding_after: None,
            new_title: String::new(),
            editing: None,
            selected_theme: None,
        }
    }

    fn load() -> Self {
        let themes = vec![
            Themes::Default,
            Themes::Dark,
            Themes::Light,
            Themes::SolarizedDark,
            Themes::SolarizedLight,
            Themes::GruvboxDark,
            Themes::GruvboxLight,
            Themes::KanagawaWave,
            Themes::KanagawaDragon,
            Themes::KanagawaLotus,
            Themes::TokyoNight,
            Themes::TokyoNightLight,
            Themes::TokyoNightStorm,
            Themes::Moonfly,
            Themes::Nightfly,
            Themes::Nord,
            Themes::Ferra,
            Themes::Dracula,
            Themes::Oxocarbon,
        ];

        let path = match Self::data_path() {
            Some(p) => p,
            None => {
                log_error("Could not resolve home directory to load lists.");
                return Self::new_with_themes(themes);
            }
        };

        if let Ok(mut file) = fs::File::open(&path) {
            let mut data = String::new();
            if let Err(e) = file.read_to_string(&mut data) {
                log_error(&format!("Failed to read lists file {}: {e}", path.display()));
                return Self::new_with_themes(themes);
            }
            match serde_json::from_str::<Self>(&data) {
                Ok(mut app) => {
                    app.themes = themes.clone();
                    app.adding_after = None;
                    app.new_title.clear();
                    app.editing = None;
                    app
                }
                Err(e) => {
                    log_error(&format!("Failed to parse lists file {}: {e}", path.display()));
                    Self::new_with_themes(themes)
                }
            }
        } else {
            let old = if let Ok(home) = std::env::var("HOME") {
                Path::new(&home).join("Tasks").join("todo.json")
            } else if let Ok(userprofile) = std::env::var("USERPROFILE") {
                Path::new(&userprofile).join("Tasks").join("todo.json")
            } else {
                PathBuf::new()
            };

            if old.exists() {
                let mut data = String::new();
                if let Ok(mut f) = fs::File::open(&old) {
                    let _ = f.read_to_string(&mut data);
                    if let Ok(mut legacy_tasks) = serde_json::from_str::<Tasks>(&data) {
                        if legacy_tasks.title.trim().is_empty() {
                            legacy_tasks.title = "Unnamed".to_string();
                        }
                        let app = List { lists: vec![legacy_tasks], themes: themes.clone(), ..Self::new() };
                        app.save();
                        return app;
                    }
                }
            }

            if let Err(e) = Self::ensure_parent_dir(&path) {
                log_error(&format!("Failed to prepare data directory {}: {e}", path.display()));
            }
            Self::new_with_themes(themes)
        }
    }

    pub fn app_theme(&self) -> Theme {
        match self.selected_theme.unwrap_or(Themes::Default) {
            Themes::Default => Theme::default(),
            Themes::KanagawaWave => Theme::KanagawaWave,
            Themes::Dark => Theme::Dark,
            Themes::Light => Theme::Light,
            Themes::Nord => Theme::Nord,
            Themes::SolarizedDark => Theme::SolarizedDark,
            Themes::SolarizedLight => Theme::SolarizedLight,
            Themes::Ferra => Theme::Ferra,
            Themes::Dracula => Theme::Dracula,
            Themes::KanagawaDragon => Theme::KanagawaDragon,
            Themes::KanagawaLotus => Theme::KanagawaLotus,
            Themes::Moonfly => Theme::Moonfly,
            Themes::Nightfly => Theme::Nightfly,
            Themes::Oxocarbon => Theme::Oxocarbon,
            Themes::TokyoNight => Theme::TokyoNight,
            Themes::TokyoNightLight => Theme::TokyoNightLight,
            Themes::TokyoNightStorm => Theme::TokyoNightStorm,
            Themes::GruvboxDark => Theme::GruvboxDark,
            Themes::GruvboxLight => Theme::GruvboxLight,
        }
    }

    pub fn update(&mut self, msg: Message) {
        match msg.clone() {
            Message::AddListAfter(index) => {
                self.adding_after = Some(index);
                self.new_title.clear();
            }
            Message::UpdateListTitle(title) => {
                self.new_title = title;
            }
            Message::ConfirmAddList => {
                let title = self.new_title.trim().to_string();
                if !title.is_empty() {
                    let new_list = Tasks { title, ..Tasks::default() };
                    let insert_at = self.adding_after.unwrap_or(self.lists.len());
                    if insert_at >= self.lists.len() {
                        self.lists.push(new_list);
                    } else {
                        self.lists.insert(insert_at + 1, new_list);
                    }
                    self.save();
                }
                self.new_title.clear();
                self.adding_after = None;
            }
            Message::CancelAddList => {
                self.new_title.clear();
                self.adding_after = None;
            }
            Message::RemoveList(index) => {
                if index < self.lists.len() {
                    self.lists.remove(index);
                    self.save();
                }
            }
            Message::ChangeListTitle(index) => {
                if self.lists.get(index).is_some() {
                    self.editing = Some(index);
                }
            }
            Message::ConfirmListEdit => {
                if let Some(index) = self.editing.take() {
                    if let Some(list) = self.lists.get_mut(index) {
                        list.title = self.new_title.clone();
                    }
                    self.new_title.clear();
                    self.save();
                }
            }
            Message::CancelListEdit => {
                self.editing = None;
                self.new_title.clear();
            }
            Message::SelectList(index) => {
                if index < self.lists.len() {
                    self.selected = Some(index);
                }
            }
            Message::BackToLists => {
                self.selected = None;
                self.adding_after = None;
                self.new_title.clear();
                self.editing = None;
            }

            Message::ThemeChanged(theme) => {
                self.selected_theme = Some(theme);
                self.save();
            }

            _ => {
                if let Some(sel) = self.selected {
                    if let Some(list) = self.lists.get_mut(sel) {
                        list.update(msg);
                        self.save();
                    }
                }
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        if let Some(sel) = self.selected {
            let header = row![
                container(text(format!("{}:", self.lists[sel].title)).size(48)).padding(16),
                text(format!("{}/{}",
                    self.lists[sel].list.iter()
                        .filter(|t| t.status == Status::Complete).count(),
                    self.lists[sel].list.len())).size(48),
            ]
            .padding(16)
            .align_y(Alignment::Center);

            let mut col = column![
                header,
                row![
                    container(button("Back to menu").on_press(Message::BackToLists)).padding(16),
                    horizontal_space(),
                    container(pick_list(self.themes.clone(), self.selected_theme, Message::ThemeChanged)
                            .placeholder("Theme..."))
                        .padding(16)
                        .align_x(Alignment::End)
                ],
                container(Rule::horizontal(1)).width(Fill)
            ];
            let tasks_ui = self.lists[sel].view();
            col = col.push(tasks_ui);
            col.into()
        } else {
            let mut root = column![
                row![
                    container(text("Lists").size(48)).padding(16),
                    horizontal_space(),
                    container(pick_list(self.themes.clone(), self.selected_theme, Message::ThemeChanged)
                        .placeholder("Theme...")).align_x(Alignment::End)
                ]
                .padding(16)
                .align_y(Alignment::Center)
            ]
            .spacing(16);

            root = root.push(container(Rule::horizontal(1)).width(Fill));

            let mut interface = column![].spacing(16).padding(16);

            if self.lists.is_empty() {
                if self.adding_after == Some(0) {
                    interface = interface.push(
                        row![
                            text_input("New list title...", &self.new_title)
                                .on_input(Message::UpdateListTitle)
                                .padding(8)
                                .width(Fill),
                            button("Save").on_press(Message::ConfirmAddList),
                            button("Cancel").on_press(Message::CancelAddList),
                        ]
                        .spacing(8),
                    );
                } else {
                    interface = interface.push(button("Add List").on_press(Message::AddListAfter(0)));
                }
            }

            for (i, lst) in self.lists.iter().enumerate() {
                if i > 0 {
                    interface = interface.push(container(Rule::horizontal(1)).width(Fill));
                }

                let row_line = row![
                    text(&lst.title).size(30).wrapping(Wrapping::Word).width(FillPortion(4)),
                    button("Select").on_press(Message::SelectList(i)),
                    button("Edit").style(button::secondary).on_press(Message::ChangeListTitle(i)),
                    button("Remove").style(button::danger).on_press(Message::RemoveList(i)),
                ]
                .spacing(12)
                .align_y(Alignment::Center);

                interface = interface.push(container(row_line).padding(8));

                if self.adding_after == Some(i) {
                    interface = interface.push(
                        row![
                            text_input("New list title...", &self.new_title)
                                .on_input(Message::UpdateListTitle)
                                .padding(8)
                                .width(Fill),
                            button("Save").on_press(Message::ConfirmAddList),
                            button("Cancel").on_press(Message::CancelAddList),
                        ]
                        .spacing(8)
                        .padding(4),
                    );
                }

                if self.editing == Some(i) {
                    interface = interface.push(
                        row![
                            text_input("New list title...", &self.new_title)
                                .on_input(Message::UpdateListTitle)
                                .padding(8)
                                .width(Fill),
                            button("Save").on_press(Message::ConfirmListEdit),
                            button("Cancel").on_press(Message::CancelListEdit),
                        ]
                        .spacing(8),
                    );
                }
            }

            if !self.lists.is_empty() {
                let end_index = self.lists.len();
                if self.adding_after == Some(end_index) {
                    interface = interface.push(
                        row![
                            text_input("New list title...", &self.new_title)
                                .on_input(Message::UpdateListTitle)
                                .padding(8)
                                .width(Fill),
                            button("Save").on_press(Message::ConfirmAddList),
                            button("Cancel").on_press(Message::CancelAddList),
                        ]
                        .spacing(8)
                        .padding(4),
                    );
                } else {
                    interface = interface.push(
                        button("Add List")
                            .style(button::secondary)
                            .on_press(Message::AddListAfter(end_index)),
                    );
                }
            }

            let scrollable_lists = scrollable(interface.spacing(12)).height(Fill);

            root.push(scrollable_lists).height(Fill).into()
        }
    }
}

impl Default for List {
    fn default() -> Self {
        Self::load()
    }
}