use eframe;
use serde;


use self::mode::graphing;

mod mode;
mod data;
mod theme_updater;


#[derive(serde::Deserialize, serde::Serialize, PartialEq)]
enum Mode {
    Simple,
    Advanced,
    Graphing,
}

impl Default for Mode {
    fn default() -> Self {
        Self::Simple
    }
}

#[derive(serde::Deserialize, serde::Serialize, PartialEq)]
enum Theme{
    Light,
    Dark,
    Nord,
}

impl Default for Theme {
    fn default() -> Self {
        Self::Nord
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
struct Size {
    text_size: f32,
    scale: f32,
}

impl Default for Size {
    fn default() -> Self {
        Size {
            text_size: 20.,
            scale: 1.,
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
struct Settings {
    mode: Mode,
    theme: Theme,
    size: Size,
}

impl Default for Settings {
    fn default() -> Self {
        Self { mode: Default::default(), theme: Default::default(), size: Default::default()}
    }
}

#[derive(serde::Deserialize, serde::Serialize, Default)]
pub struct CalcState {
    data: data::Data,
    settings: Settings,
}

impl CalcState {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        
        theme_updater::load_theme(&cc.egui_ctx);
        
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }
        Default::default()
    }
}


impl eframe::App for CalcState {
    
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
    
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        
        let Self { data, settings} = self;
        
        theme_updater::update_theme(ctx);
        
        eframe::egui::TopBottomPanel::top("menu").show(ctx, |ui| {
            eframe::egui::menu::bar(ui, |ui| {
                ui.menu_button("Mode", |ui| {
                    #[cfg(not(target_arch = "wasm32"))]
                    if ui.button("Quit").clicked() {
                        _frame.close();
                    }
                    ui.radio_value(&mut settings.mode, Mode::Simple, "Simple");
                    ui.radio_value(&mut settings.mode, Mode::Advanced, "Advanced");
                    ui.radio_value(&mut settings.mode, Mode::Graphing, "Graphing");
                });
                ui.menu_button("UI", |ui| {
                    if ui.button("Reload Theme").clicked() {
                        theme_updater::load_theme(ctx);
                    }
                    ui.add(eframe::egui::Slider::new(&mut settings.size.text_size, 10.0..=40.0).text("Text Size").step_by(1.));
                    ui.add(eframe::egui::Slider::new(&mut settings.size.scale, 0.5..=2.0).text("App Scale").step_by(0.25));
                });
            });
        });
        
        match settings.mode {
            Mode::Simple => {
                mode::simple::simple(data, ctx)
            }
            Mode::Advanced => {
                mode::advanced::advanced(data, ctx)
            }
            Mode::Graphing => {
                graphing::graphing(data, ctx)
            }
        }
    }
}