use egui::Ui;

#[derive(Clone)]
pub struct NewProjectState {
    name: String,
    path: String,
    cancelled: bool,
    submitted: bool,
}

impl Default for NewProjectState {
    fn default() -> Self {
        Self {
            cancelled: false,
            submitted: false,
            name: "".to_string(),
            path: "".to_string(),
        }
    }
}

impl NewProjectState {
    pub fn new() -> Self { Default::default() }
    pub fn name(&self) -> &String { &self.name }
    pub fn path(&self) -> &String { &self.path }
    pub fn is_cancelled(&self) -> bool {
        self.cancelled
    }
    pub fn is_submitted(&self) -> bool {
        self.submitted
    }
}

pub fn new_project(ui: &mut Ui, state: &NewProjectState) -> NewProjectState {
    let mut state = state.to_owned();
    ui.centered_and_justified(|ui| {
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.label("Name ");
                ui.text_edit_singleline(&mut state.name);
            });
            ui.horizontal(|ui| {
                ui.label("Project Path");
                ui.text_edit_singleline(&mut state.path);
                if ui.button("Open").clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_folder() {
                        state.path = path.to_string_lossy().to_string();
                    }
                }
            });
            ui.horizontal(|ui| {
                if ui.button("Cancel").clicked() {
                    state.cancelled = true;
                }
                if ui.button("Create").clicked() {
                    state.submitted = true;
                }
            });
        })
    });
    state
}
