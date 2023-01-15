use eframe::epaint::Vec2;

use crate::app::data;

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
                        if let Ok(parsed_value) = f64_wrapper.parse::<f64>() {
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
                if ui.button("Add New").clicked() {
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
                    let mut variables_substituted = data.input_output.input.clone();
                    for variable in data.saved_values.clone() {
                        variables_substituted = variables_substituted.as_str().replace(variable.0.as_str(), format!("( {} ), ", variable.1).as_str());
                    }
                    let mut yard = rustyard::ShuntingYard::new();
                    if let Ok(output) = yard.calculate(variables_substituted.as_str()) {
                        data.input_output.output = output.to_string()
                    }
                    data.history.push(data.input_output.clone());
                }
                if ui.add(eframe::egui::Button::new("*").min_size(Vec2 { x: 75., y: 75. })).clicked() {
                    data.input_output.input.push_str("*");
                }
            });
        })
        
    });
}