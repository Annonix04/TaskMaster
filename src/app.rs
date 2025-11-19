#![allow(unused_imports, dead_code)]
use crate::models::*;
use iced::{
    self, 
    Alignment, 
    Element, 
    Fill, 
    widget::*, 
    widget::{
        column,
        container
    }, 
    Length
};
use serde::{
    Deserialize, 
    Serialize
};
use std::{
    fs,
    io::{
        self, 
        Read, 
        Write
    },
    path::{
        Path, 
        PathBuf
    },
};
use std::fmt::{
    Display, 
    Formatter
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
    fn data_path() -> Option<PathBuf> {
        if let Ok(home) = std::env::var("HOME") {
            return Some(Path::new(&home).join("Tasks").join("todo.json"));
        }
        if let Ok(userprofile) = std::env::var("USERPROFILE") {
            return Some(Path::new(&userprofile).join("Tasks").join("todo.json"));
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
                            "Failed to write tasks to {}: {e}",
                            path.display()
                        ));
                    }
                }
                Err(e) => log_error(&format!("Failed to serialize tasks: {e}")),
            }
        } else {
            log_error("Could not resolve home directory to save tasks.");
        }
    }

    fn load() -> Self {
        if let Some(path) = Self::data_path() {
            if let Ok(mut file) = fs::File::open(&path) {
                let mut data = String::new();
                if let Err(e) = file.read_to_string(&mut data) {
                    log_error(&format!(
                        "Failed to read tasks file {}: {e}",
                        path.display()
                    ));
                    return Self {
                        list: vec![],
                        adding_after: None,
                        new_title: String::new(),
                        selected_theme: None,
                        themes: vec![
                            Themes::Default,
                            Themes::KanagawaWave,
                            Themes::Dark,
                            Themes::Light,
                            Themes::Nord,
                            Themes::SolarizedDark,
                            Themes::SolarizedLight
                        ]
                    };
                }
                match serde_json::from_str::<Self>(&data) {
                    Ok(mut tasks) => {
                        tasks.adding_after = None;
                        tasks.new_title.clear();
                        tasks
                    }
                    Err(e) => {
                        log_error(&format!(
                            "Failed to parse tasks file {}: {e}",
                            path.display()
                        ));
                        Self {
                            list: vec![],
                            adding_after: None,
                            new_title: String::new(),
                            selected_theme: None,
                            themes: vec![
                                Themes::Default,
                                Themes::KanagawaWave,
                                Themes::Dark,
                                Themes::Light,
                                Themes::Nord,
                                Themes::SolarizedDark,
                                Themes::SolarizedLight
                            ]
                        }
                    }
                }
            } else {
                if let Err(e) = Self::ensure_parent_dir(&path) {
                    log_error(&format!(
                        "Failed to prepare data directory {}: {e}",
                        path.display()
                    ));
                }
                Self {
                    list: vec![],
                    adding_after: None,
                    new_title: String::new(),
                    selected_theme: None,
                    themes: vec![
                        Themes::Default,
                        Themes::KanagawaWave,
                        Themes::Dark,
                        Themes::Light,
                        Themes::Nord,
                        Themes::SolarizedDark,
                        Themes::SolarizedLight
                    ]
                }
            }
        } else {
            log_error("Could not resolve home directory to load tasks.");
            Self {
                list: vec![],
                adding_after: None,
                new_title: String::new(),
                selected_theme: None,
                themes: vec![
                    Themes::Default,
                    Themes::KanagawaWave,
                    Themes::Dark,
                    Themes::Light,
                    Themes::Nord,
                    Themes::SolarizedDark,
                    Themes::SolarizedLight
                ]
            }
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
        }
    }

    pub fn update(&mut self, msg: Message) {
        match msg {
            Message::AddAfter(index) => {
                self.adding_after = Some(index);
                self.new_title.clear();
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
                    self.save();
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
                    self.save();
                }
            }
            Message::Forward(index) => {
                if let Some(task) = self.list.get_mut(index) {
                    task.update(Message::Forward(index));
                    self.save();
                }
            }
            Message::ThemeChanged(theme) => {
                self.selected_theme = Some(theme);
                self.save();
            }
        }
    }

    pub fn view(&self) -> Element<Message> {
        let mut root = column![
            row![
                container(text("Tasks:").size(48)).padding(16),
                horizontal_space(),
                container(pick_list(self.themes.clone(), self.selected_theme, Message::ThemeChanged)
                    .placeholder("Theme..."))
                .align_x(Alignment::End)
            ]
            .padding(16)
            .align_y(Alignment::Center)
        ]
            .spacing(16);

        root = root.push(container(Rule::horizontal(1)).width(Fill));

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

            if self.adding_after == Some(i) {
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
                interface = interface.push(button("Add Task").on_press(Message::AddAfter(end_index)));
            }
        }
        let scrollable_list = scrollable(interface.spacing(12)).height(Fill);

        root.push(scrollable_list).height(Fill).into()
    }
}

impl Default for Tasks {
    fn default() -> Self {
        Self::load()
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

    fn view(&self, id: usize) -> Element<Message> {
        let mut interface = row![
            text(&self.title).size(20),
            text(format!(" - {:?}", self.status)).size(16),
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

        interface = interface.push(button("Remove").on_press(Message::Remove(id)));

        container(interface).padding(4).width(Fill).into()
    }
}
