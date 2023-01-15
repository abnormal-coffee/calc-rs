use eframe::{self, epaint::Vec2};

use crate::app::data;

pub fn simple(data: &mut data::Data, ctx: &eframe::egui::Context,) {
    
    eframe::egui::CentralPanel::default().show(ctx, |ui| {

        ui.add(eframe::egui::TextEdit::multiline(&mut data.input_output.input).desired_rows(5));
        ui.separator();
        ui.add(eframe::egui::TextEdit::singleline(&mut data.input_output.output).hint_text("Output"));
        
        eframe::egui::containers::Frame::none().show(ui, |ui| {
            eframe::egui::Grid::new("simple").show(ui, |ui| {
                for i in 0..=2 {
                    for n in 1..=3 {
                        let str = format!("{}", (3 * i + n));
                        if ui.add(eframe::egui::Button::new(str).min_size(Vec2{x: 75., y: 75.})).clicked() {
                            data.input_output.input.push(format!("{}", (3 * i + n)).chars().nth(0).unwrap());
                        }
                    }
                    match i {
                        0 => {if ui.add(eframe::egui::Button::new("-").min_size(Vec2 { x: 75., y: 75. })).clicked() {
                            data.input_output.input.push_str("-");
                        }}
                        1 => {if ui.add(eframe::egui::Button::new("+").min_size(Vec2 { x: 75., y: 75. })).clicked() {
                            data.input_output.input.push_str("+");
                        }}
                        2 => {if ui.add(eframe::egui::Button::new("÷").min_size(Vec2 { x: 75., y: 75. })).clicked() {
                            data.input_output.input.push_str("/");
                        }}
                        _ => {println!("There is an error with the ui")}
                    }
                    ui.end_row();
                }
                if ui.add(eframe::egui::Button::new("Clear").min_size(Vec2 { x: 75., y: 75. })).clicked() {
                    data.input_output.input = String::new();
                }
                if ui.add(eframe::egui::Button::new("0").min_size(Vec2{x: 75., y: 75.})).clicked() {
                    data.input_output.input.push_str("0");
                }
                if ui.add(eframe::egui::Button::new("=").min_size(Vec2 { x: 75., y: 75. })).clicked() {
                    let mut yard = rustyard::ShuntingYard::new();
                    data.input_output.output = yard.calculate(data.input_output.input.as_str()).unwrap().to_string();
                    data.history.push(data.input_output.clone());
                }
                if ui.add(eframe::egui::Button::new("*").min_size(Vec2 { x: 75., y: 75. })).clicked() {
                    data.input_output.input.push_str("*");
                }
            });
        })
        
    });
}