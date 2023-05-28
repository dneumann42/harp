use egui::Id;

pub enum Kind {
    Function,
    Type,
    Module,
}

pub trait Editor {
    fn uid(&self) -> Id;
    fn is_visible(&self) -> bool;
    fn hide(&mut self);
    fn show(&mut self);
    fn toggle(&mut self) { if self.is_visible() { self.hide() } else { self.show() } }
    fn kind() -> Kind;
}