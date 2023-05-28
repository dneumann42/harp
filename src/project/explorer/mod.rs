use std::os::linux::raw::stat;
use egui::{AboveOrBelow, Id, popup_above_or_below_widget, Ui, Window};
use uuid::Uuid;
use crate::nodes::functions::Function;
use crate::project::buffer::{Buffer, View};
use crate::project::function_editor::FunctionEditor;
use crate::project::Project;

pub struct ExplorerState {
    function_editors: Vec<FunctionEditor>,
}

impl ExplorerState {
    pub fn new() -> Self {
        Self {
            function_editors: vec![],
        }
    }

    fn update_function_editors(&mut self, ui: &mut Ui) {
        for it in &mut self.function_editors {
            it.show(&ui.ctx())
        }
    }

    fn add_function_editor(&mut self, id: Id, fun: Option<Function>) {
        println!("{:?}", id);
        self.function_editors.push(FunctionEditor::new(id, fun))
    }
}

pub fn explorer_ui(state: &mut ExplorerState, project: &Project, ui: &mut Ui) {
    state.update_function_editors(ui);

    ui.label("Nodes:");

    let new_node_menu_id = ui.make_persistent_id("new-node-menu-id");

    ui.horizontal(|ui| {
        let response = ui.button(egui::RichText::new("New"));

        if response.clicked() {
            ui.memory_mut(|mem| mem.toggle_popup(new_node_menu_id));
        }

        popup_above_or_below_widget(ui, new_node_menu_id, &response, AboveOrBelow::Below, |ui| {
            ui.set_min_width(100.0);
            ui.vertical(|ui| {
                if ui.button("Function").clicked() {
                    ui.memory_mut(|mem| mem.toggle_popup(new_node_menu_id));
                    state.add_function_editor(ui.make_persistent_id(Uuid::new_v4()), None);
                }

                if ui.button("Type").clicked() {
                    ui.memory_mut(|mem| mem.toggle_popup(new_node_menu_id));
                }

                if ui.button("Module").clicked() {
                    ui.memory_mut(|mem| mem.toggle_popup(new_node_menu_id));
                }
            })
        });
    });

    ui.vertical(|ui| {
        for (name, _) in &project.env().functions() {
            let _ = ui.button(name);
        }
    });
}