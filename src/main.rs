#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

use iced;
use iced::Size;

use TaskMaster::Tasks;

fn main() -> iced::Result {
    iced::application("Tasks", Tasks::update, Tasks::view)
        .window_size(Size::new(1000.0, 700.0))
        .centered()
        .resizable(false)
        .run()
}
