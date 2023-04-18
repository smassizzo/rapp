use crate::screen::{Screen, ScreenHandle};
use egui::{Response, Ui};

pub trait Draw {
    fn draw(&self, screen: &mut Screen) -> ScreenHandle;
}

pub(crate) trait EguiDraw {
    fn draw_egui(&self, ui: &mut Ui) -> Response;
}
