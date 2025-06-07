use dioxus::desktop::{Config, LogicalSize, WindowBuilder};

mod app;
mod routes;
mod models;

fn main() {
    let window_builder = WindowBuilder::new()
        .with_title("LoL Profile Finder")
        .with_resizable(false)
        .with_always_on_top(false)
        .with_maximizable(false)
        .with_inner_size(LogicalSize::new(600.0, 600.0));

    let config = Config::default()
        .with_menu(None)
        .with_window(window_builder);

    dioxus::LaunchBuilder::desktop()
        .with_cfg(config)
        .launch(app::App)
}