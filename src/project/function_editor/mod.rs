use egui::{Context, Id, Ui, Window};
use uuid::Uuid;
use crate::nodes::functions::Function;
use crate::project::buffer::{Buffer, BufferView, View};
use crate::project::editor::{Editor, Kind};

pub struct FunctionEditor {
    visible: bool,
    code: String,
    id: Id,
}

impl FunctionEditor {
    pub fn new(id: Id, fun: Option<Function>) -> Self {
        Self {
            id,
            visible: true,
            code: fun.map_or("".to_string(), |f| f.to_string()),
        }
    }
}

impl Buffer for FunctionEditor {
    fn name(&self) -> &'static str {
        "🖮 Function Editor"
    }

    fn show(&mut self, ctx: &egui::Context) {
        let mut v = self.visible;
        Window::new(self.name())
            .open(&mut v)
            .id(self.id)
            .min_width(480.0)
            .min_height(320.0)
            .resizable(true)
            .show(ctx, |ui| self.ui(ui));
        self.visible = v
    }
}

impl View for FunctionEditor {
    fn ui(&mut self, ui: &mut Ui) {
        let _ = ui.button("CLICK-ME");
    }
}

impl BufferView for FunctionEditor {}

impl Editor for FunctionEditor {
    fn uid(&self) -> Id {
        self.id
    }

    fn is_visible(&self) -> bool {
        self.visible
    }

    fn hide(&mut self) {
        self.visible = false
    }

    fn show(&mut self) {
        self.visible = true
    }

    fn kind() -> crate::project::editor::Kind {
        Kind::Function
    }
}
