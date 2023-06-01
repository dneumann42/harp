use egui::{Color32, Context, Id, ScrollArea, Ui, Window};
use egui::WidgetText::RichText;
use egui::WidgetType::TextEdit;
use uuid::Uuid;
use crate::nodes::functions::{Call, Function};
use crate::nodes::Node;
use crate::project::buffer::{Buffer, BufferView, View};
use crate::project::editor::{Editor, Kind};
use crate::reader::{ParseErr, read, read_node};

pub struct FunctionEditor {
    visible: bool,
    code: String,
    fun: Option<Function>,
    error: Option<ParseErr>,
    id: Id,
}

impl FunctionEditor {
    pub fn new(id: Id, fun: Option<Function>) -> Self {
        Self {
            id,
            fun: fun.clone(),
            visible: true,
            error: None,
            code: fun.map_or("".to_string(), |f| f.to_string()),
        }
    }

    pub fn update(&mut self) {
        match read_node(&self.code) {
            Ok(ok) if matches!(&ok, Node::Call(_)) => {
                match &ok {
                    Node::Call(call) => {
                        self.fun = Some(call.as_fun())
                    }
                    _ => {}
                }
                self.error = None
            }
            Err(err) => {
                self.error = Some(err)
            }
            v => {
                self.error = Some(
                    ParseErr::ExpectedFunction(
                        format!("Expected function but got {:?}", v)
                    )
                )
            }
        }
    }

    pub fn get_code(&self) -> &String {
        &self.code
    }

    pub fn get_fun(&self) -> &Option<Function> {
        &self.fun
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
        ui.horizontal(|ui| {
            let _ = ui.button("▶");
            let _ = ui.button("test");
        });

        ScrollArea::vertical().show(ui, |ui| {
            ui.add(
                egui::TextEdit::multiline(&mut self.code)
                    .font(egui::TextStyle::Monospace)
                    .code_editor()
                    .desired_rows(10)
                    .lock_focus(true)
            )
        });

        if let Some(err) = &self.error {
            ui.vertical(|ui| {
                ui.label(egui::RichText::new("Error").color(Color32::from_rgb(100, 55, 33)).size(20.0));
                ui.label(err.to_string());
            });
        }

        self.update();
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
