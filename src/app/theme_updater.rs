use std::{sync::mpsc::channel, time::Duration};

use eframe;
use ron;
use home;

pub fn update_theme(ctx: &eframe::egui::Context) {
    // To do
}

pub fn load_theme(ctx: &eframe::egui::Context) {
    println!("Attempting to load theme");
    let path = format!("{}/.egui-theme", home::home_dir().unwrap().to_str().unwrap());
    println!("Path set to: {}", path);
    if let Ok(file) = std::fs::read_to_string(path) {
        println!("File succesfully loaded");
        let theme: ron::Result<eframe::egui::Visuals, ron::error::SpannedError>  = ron::from_str(file.as_str());
        if let Ok(theme) = theme.clone() {
            ctx.set_visuals(theme);
            println!("Set theme")
        }
        if let Err(_) = theme.clone() {
            println!("An error occurred loading the theme");
        }
    }
}
