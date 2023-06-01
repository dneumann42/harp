use egui::{Context, Ui};

pub trait View {
    fn ui(&mut self, ui: &mut Ui);
}

pub trait Buffer {
    fn name(&self) -> &'static str;
    fn show(&mut self, ctx: &Context);
}

pub trait BufferView: Buffer + View {}