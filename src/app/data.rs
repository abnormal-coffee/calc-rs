use eframe::egui::plot::PlotPoint;
use serde;
use eframe;
use eframe::epaint::Color32;


#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct InputOutput {
    pub input: String,
    pub output: String,
}

impl Default for InputOutput {
    fn default() -> Self {
        InputOutput{
            input: String::new(),
            output: String::new(),
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Funtion {
    colour: Color32,
    
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Data {
    pub input_output: InputOutput,
    pub history: Vec<InputOutput>,
    pub saved_values: Vec<(String, f32)>,
    pub saved_functions: Vec<Vec<String>>,
    pub graphs: Vec<Graph>,
    pub remove: (usize, bool),
}

impl Default for Data {
    fn default() -> Self {
        Data {
            input_output: InputOutput::default(),
            history: Vec::new(),
            saved_values: Vec::new(),
            saved_functions: Vec::new(),
            graphs: vec![Graph::default()],
            remove: (0, false),
        }
    }
}

#[derive(PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Graph {
    pub function: String,
    pub animate: bool,
    pub colour: Color32,
    pub name: String,
    pub precision: f64,
    pub total_points: u64,
    #[serde(skip)]
    pub plot_points: Vec<PlotPoint>,
}

impl Default for Graph {
    fn default() -> Self {
        Self {
            function: String::from("(x ^ 2) / (10)"),
            animate: false,
            colour: Color32::TRANSPARENT,
            name: String::from("Default"),
            precision: 10.,
            total_points: 1000,
            plot_points: Vec::new(),
        }
    }
}