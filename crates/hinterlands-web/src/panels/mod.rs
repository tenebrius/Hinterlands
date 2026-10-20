//! Side panels. A panel reads `ViewState` and returns messages; it never touches
//! the simulation or another panel. Adding one is a new file and a line in `all()`.

mod controls;

use crate::view::{Msg, ViewState};

pub trait Panel {
    fn title(&self) -> &'static str;
    fn default_open(&self) -> bool {
        true
    }
    fn ui(
        &mut self,
        ui: &mut egui::Ui,
        view: &ViewState,
        layers: &[(&'static str, &'static str)],
    ) -> Vec<Msg>;
}

pub fn all() -> Vec<Box<dyn Panel>> {
    vec![Box::new(controls::Controls::default())]
}
