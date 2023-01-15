use asciimath::{Scope, eval};
use eframe::epaint::Vec2;

use crate::app::data::{self, Data};

pub fn advanced(data: &mut data::Data, ctx: &eframe::egui::Context,) {
    
    eframe::egui::SidePanel::right("advanced stuff").show(ctx, |ui| {
        eframe::egui::ScrollArea::vertical().show(ui, |ui| {
            ui.collapsing("History", |ui| {
                for i in 0..data.history.len() {
                    ui.horizontal(|ui| {
                        ui.label(data.history[i].input.clone());
                        ui.label(data.history[i].output.clone());
                    });
                }
                if data.history.len() > 0 {
                    if ui.button("Clear").clicked() {
                        data.history = Vec::new();
                    }
                }
            });
            ui.separator();
            ui.collapsing("Variables", |ui| {
                for i in 0..data.saved_values.len() {
                    ui.horizontal(|ui| {
                        let f64_wrapper = &mut data.saved_values[i].1.clone().to_string();
                        let name_wrapper = &mut data.saved_values[i].0.clone();
                        ui.add(eframe::egui::TextEdit::singleline(name_wrapper).hint_text("Variable Name").desired_width(100.));
                        ui.add(eframe::egui::TextEdit::singleline(f64_wrapper).hint_text("Variable Value").desired_width(100.));
                        if let Ok(parsed_value) = f64_wrapper.parse::<f32>() {
                            data.saved_values[i] = (name_wrapper.clone(), parsed_value);
                        }
                        else {data.saved_values[i] = (name_wrapper.clone(), 0.)}
                        if ui.button("x").clicked() {
                            data.remove = (i, true);
                        }
                    });
                }
                if data.remove.1 == true {
                    data.saved_values.remove(data.remove.0);
                    data.remove.1 = false;
                }
                if ui.button("Add New                                                              ").clicked() {
                    data.saved_values.push(("name".to_string(), 0.));
                }
            });
        }); 
    });
    
    eframe::egui::CentralPanel::default().show(ctx, |ui| {

        ui.add(eframe::egui::TextEdit::multiline(&mut data.input_output.input).hint_text("Input - if you press enter it will create a new line").desired_rows(1).desired_width(315.));
        ui.add(eframe::egui::TextEdit::singleline(&mut data.input_output.output).hint_text("Output").desired_width(315.));
        
        ui.separator();
        
        eframe::egui::containers::Frame::none().show(ui, |ui| {
            eframe::egui::Grid::new("Advanced").show(ui, |ui| {
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