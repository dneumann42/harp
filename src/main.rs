#[cfg_attr(not(debug_assertions), window_supsystem = "windows")]
use evaluator::eval_node;
use reader::read;
use gui::new_project::*;
use rustyline::{error::ReadlineError, DefaultEditor};

use eframe::egui;
use egui::{Color32, Direction};
use harp::project::Project;
use crate::HarpAppState::NewProject;

pub mod evaluator;
pub mod gui;
pub mod nodes;
pub mod reader;

fn repl() {
    let mut rl = DefaultEditor::new().unwrap();

    #[cfg(feature = "with-file-history")]
    if rl.load_history(config_dir()).is_err() {
        //
    }

    loop {
        let readline = rl.readline("> ");

        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str()).unwrap();
                let node = read(line).unwrap();
                println!("{:?}", eval_node(node));
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
}

#[derive(Clone)]
enum HarpAppState {
    Main,
    Edit,
    NewProject(NewProjectState),
}

impl From<NewProjectState> for HarpAppState {
    fn from(value: NewProjectState) -> Self {
        HarpAppState::NewProject(value)
    }
}

#[derive(Clone)]
struct HarpApplication {
    state: HarpAppState,
    project: Option<Project>,
}

impl HarpApplication {
    pub fn new() -> Self { Default::default() }
    pub fn set_state(&mut self, state: HarpAppState) {
        self.state = state
    }
}

impl Default for HarpApplication {
    fn default() -> Self {
        Self {
            state: HarpAppState::Main,
            project: None,
        }
    }
}

impl eframe::App for HarpApplication {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            match self.state.clone() {
                HarpAppState::Main => {
                    ui.vertical_centered_justified(|ui| {
                        ui.colored_label(Color32::LIGHT_GRAY, "Hello, World");
                        if ui.button("Open Project").clicked() {}
                        if ui.button("New Project").clicked() {
                            self.set_state(NewProject(Default::default()));
                            return;
                        }
                    });
                }
                NewProject(state) => {
                    if state.is_cancelled() {
                        self.set_state(HarpAppState::Main);
                        return;
                    }
                    if state.is_submitted() {
                        let mut proj = Project::make(state.name(), state.path());
                        proj.save();
                        self.project = Some(proj);
                        self.set_state(HarpAppState::Edit);
                        return;
                    }
                    self.set_state(NewProject(new_project(ui, state)))
                }
                HarpAppState::Edit => {
                    match &self.project {
                        None => {}
                        Some(proj) => {
                            ui.label(proj.name());
                        }
                    }
                }
            }
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(480.0, 320.0)),
        ..Default::default()
    };

    eframe::run_native(
        "Harp",
        options,
        Box::new(|_| Box::<HarpApplication>::default()),
    )
}
