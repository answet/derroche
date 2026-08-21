#![windows_subsystem = "windows"]
mod database;
mod formato;
mod models;
mod repository;
mod gui;
mod estilos;
mod updater;

fn main() -> iced::Result {
    gui::run()
}
