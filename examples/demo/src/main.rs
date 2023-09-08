mod dashboard;
mod detail;
mod group_new;
mod state;

use crate::dashboard::Dashboard;
use eframe::egui;
use log::LevelFilter;

fn main() -> Result<(), eframe::Error> {
    env_logger::builder().filter_level(LevelFilter::Trace); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(1280.0, 1024.0)),
        resizable: false,
        centered: true,
        // decorated: false,
        follow_system_theme: true,
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_cc| Box::<Dashboard>::default()),
    )
}
