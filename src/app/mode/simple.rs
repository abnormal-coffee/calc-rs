use asciimath::{Scope, eval};
use eframe::{self, epaint::Vec2};

use crate::app::data::{self, Data};

pub fn simple(data: &mut data::Data, ctx: &eframe::egui::Context,) {
    
    eframe::egui::CentralPanel::default().show(ctx, |ui| {

        ui.add(eframe::egui::TextEdit::multiline(&mut data.input_output.input).desired_rows(1).desired_width(315.));
        ui.add(eframe::egui::TextEdit::singleline(&mut data.input_output.output).desired_width(315.).hint_text("Output"));
        
        ui.separator();
        
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
                    evaluate(data);
                }
                if ui.add(eframe::egui::Button::new("*").min_size(Vec2 { x: 75., y: 75. })).clicked() {
                    data.input_output.input.push_str("*");
                }
            });
        })
        
    });
}

fn evaluate(data: &mut Data) {
    let expression = data.input_output.input.clone();
    let mut variables = Scope::new();
    for (name, val) in data.saved_values.clone(){
        variables.set_var::<f32>(&name, val);
    };
    if let Ok(result) = eval(&expression, &variables) {
        data.input_output.output = result.to_string()
    };
    if let Err(err) = eval(&expression, &variables) {
        data.input_output.output = err.to_string()
    };
    data.history.push(data.input_output.clone());
}