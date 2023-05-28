use std::fs;

use std::fs::write;
use std::path::{Path, PathBuf};
use directories::BaseDirs;
#[cfg_attr(not(debug_assertions), window_supsystem = "windows")]
use evaluator::eval_node;
use reader::read;
use gui::new_project::*;
use rustyline::{error::ReadlineError, DefaultEditor};

use eframe::{egui, Storage};
use eframe::glow::Context;
use egui::{Color32, Direction, RichText, WidgetText};
use serde_derive::{Deserialize, Serialize};
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

#[derive(Deserialize, Serialize, Debug, Clone)]
struct HarpAppConfig {
    last_opened_projects: Vec<String>,
}

impl HarpAppConfig {
    pub fn new() -> Self {
        Self {
            last_opened_projects: vec![]
        }
    }
}

impl From<NewProjectState> for HarpAppState {
    fn from(value: NewProjectState) -> Self {
        HarpAppState::NewProject(value)
    }
}

#[derive(Clone)]
struct HarpApplication {
    loaded: bool,
    config: HarpAppConfig,
    state: HarpAppState,
    project: Option<Project>,
}

impl HarpApplication {
    pub fn new() -> Self { Default::default() }
    pub fn set_state(&mut self, state: HarpAppState) {
        self.state = state
    }

    pub fn app_data_dir() -> Option<PathBuf> {
        match BaseDirs::new() {
            None => None,
            Some(xs) => Some(xs.config_local_dir().to_path_buf())
        }
    }

    pub fn save_cfg(&self) {
        match HarpApplication::app_data_dir() {
            None => {}
            Some(path) => {
                let s = toml::to_string(&self.config).unwrap();
                write(path.join("config.toml"), s).unwrap()
            }
        }
    }

    pub fn load_cfg(&mut self) {
        match HarpApplication::app_data_dir() {
            None => {}
            Some(path) => {
                if path.join("config.toml").exists() {
                    let contents = fs::read_to_string(path.join("config.toml")).unwrap();
                    self.config = toml::from_str(&*contents.as_str()).unwrap();
                    self.loaded = true
                }
                {
                    self.save_cfg()
                }
            }
        }
    }
}

impl Default for HarpApplication {
    fn default() -> Self {
        Self {
            loaded: false,
            config: HarpAppConfig::new(),
            state: HarpAppState::Main,
            project: None,
        }
    }
}

impl eframe::App for HarpApplication {
    fn save(&mut self, storage: &mut dyn Storage) {
        self.save_cfg()
    }

    fn on_exit(&mut self, _gl: Option<&Context>) {
        self.save_cfg();
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if !self.loaded {
            self.load_cfg();
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            match &self.state {
                HarpAppState::Main => {
                    ui.vertical_centered_justified(|ui| {
                        ui.colored_label(Color32::LIGHT_GRAY, "Harp");
                        if ui.button("Open Project").clicked() {
                            if let Some(path) = rfd::FileDialog::new().pick_folder() {
                                let pr = Project::load_project(path.to_string_lossy().to_string());
                                self.project = Option::from(pr.clone());
                                self.set_state(HarpAppState::Edit);
                                self.config.last_opened_projects.push(path.to_string_lossy().to_string());
                            }
                        }
                        if ui.button("New Project").clicked() {
                            self.set_state(NewProject(Default::default()));
                            return;
                        }
                        ui.colored_label(Color32::LIGHT_GRAY, "Recent Projects");

                        for file in &self.config.last_opened_projects {
                            ui.horizontal(|ui| {
                                ui.label(file);
                                let _ = ui.button("Edit");
                                let _ = ui.button("...");
                            });
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
                        self.config.last_opened_projects.push(proj.path());
                        proj.save();
                        self.project = Some(proj);
                        self.set_state(HarpAppState::Edit);
                        return;
                    }
                    self.set_state(NewProject(new_project(ui, state)))
                }
                HarpAppState::Edit => {
                    ui.vertical(|ui| {
                        match &self.project {
                            None => {}
                            Some(proj) => {
                                ui.label("Files:");
                                ui.vertical(|ui| {
                                    for file in &proj.files() {
                                        let _ = ui.button(file);
                                    }
                                });
                            }
                        }
                    });
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
