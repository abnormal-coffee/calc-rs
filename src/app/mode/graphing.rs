use eframe::egui::{color_picker::Alpha, plot::PlotPoints};

use crate::app::data::{self};

pub fn graphing(data: &mut data::Data, ctx: &eframe::egui::Context,) {
    eframe::egui::SidePanel::right("graphing").resizable(true).show(ctx, |ui| {
        eframe::egui::ScrollArea::vertical().show(ui, |ui| {
            ui.collapsing("funtions", |ui| {
                for i in 0..data.graphs.len() {
                    let mut update = false;
                    ui.horizontal_wrapped(|ui| {
                        ui.add(eframe::egui::TextEdit::singleline(&mut data.graphs[i].name).hint_text("Graph Name").desired_width(100.));
                        ui.add(eframe::egui::TextEdit::singleline(&mut data.graphs[i].function).hint_text("Funtion Of x").desired_width(100.));
                        ui.toggle_value(&mut data.graphs[i].animate, "Live?");
                        eframe::egui::color_picker::color_edit_button_srgba(ui, &mut data.graphs[i].colour, Alpha::Opaque);
                        if data.graphs[i].animate == false {
                            if ui.button("Update").clicked() {
                                update = true;
                            }
                        }
                    });
                    ui.horizontal_wrapped(|ui| {                    
                        ui.add(eframe::egui::Slider::new(&mut data.graphs[i].precision, 0.1..=1000.).logarithmic(true));
                        ui.add(eframe::egui::Slider::new(&mut data.graphs[i].total_points, 0..=100000).logarithmic(true));
                        if ui.button("Remove").clicked() {
                            data.remove = (i, true);
                        }
                        
                        if data.graphs[i].animate == true || update == true {
                            data.graphs[i].plot_points = Vec::new();
                            let mut x: f64 = 0.;
                            while x < data.graphs[i].precision * data.graphs[i].total_points as f64 {
                                let mut variables_substituted = data.graphs[i].function.clone();
                                variables_substituted = variables_substituted.as_str().replace("x", format!("( {} )", x).as_str());
                                // variables_substituted = variables_substituted.as_str().replace("i", format!("( {} )", data.graphs).as_str());
                                for variable in data.saved_values.clone() {
                                    variables_substituted = variables_substituted.as_str().replace(variable.0.as_str(), format!("( {} ), ", variable.1).as_str());
                                }
                                let mut yard = rustyard::ShuntingYard::new();
                                if let Ok(output) = yard.calculate(variables_substituted.as_str()) {
                                    data.graphs[i].plot_points.push(eframe::egui::plot::PlotPoint{
                                        x: x,
                                        y: output
                                    });
                                }
                                if x >= 0. {x += 1. / data.graphs[i].precision;}
                                else {x -= 1. / data.graphs[i].precision;}
                            }
                        }
                    });
                }
                if data.remove.1 == true {
                    let _ = data.graphs.remove(data.remove.0);
                    data.remove.1 = false;
                }
                if ui.button("Add New").clicked() {
                    data.graphs.push(data::Graph::default());
                }
                ui.separator();
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
                            data.remove.1 = true;
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
        eframe::egui::plot::Plot::new("Plot").view_aspect(2.0).show(ui, |plot_ui| {
            for i in 0..data.graphs.len() {
                plot_ui.line(eframe::egui::plot::Line::new(PlotPoints::Owned(data.graphs[i].plot_points.clone())).name(data.graphs[i].name.clone()).color(data.graphs[i].colour))
            };
        });
    });
}