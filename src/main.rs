#![windows_subsystem = "windows"]
mod database;
mod models;
mod repository;
mod gui;
mod estilos;
mod updater;

fn main() -> iced::Result {
    gui::run()
}
