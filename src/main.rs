#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

use iced;
use iced::Size;

use TaskMaster::models::*;

fn main() -> iced::Result {
    iced::application("TaskMaster", Tasks::update, Tasks::view)
        .theme(|s| s.app_theme())
        .window_size(Size::new(1000.0, 700.0))
        .centered()
        .resizable(false)
        .run()
}