use super::traits::{Draw, EguiDraw};
use crate::screen::{Screen, ScreenHandle};
use egui::{vec2, Color32, Rect, Response, Rounding, Sense, Ui};
use std::{cell::RefCell, rc::Rc};

#[derive(Clone, Default)]
pub struct Separator(pub(crate) Rc<RefCell<SeparatorData>>);

impl Separator {
    pub fn thickness(self, thickness: u16) -> Self {
        self.0.borrow_mut().thickness = thickness.into();
        self
    }
}

#[derive(Clone)]
pub(crate) struct SeparatorData {
    pub thickness: f32,
}

impl Default for SeparatorData {
    fn default() -> Self {
        Self { thickness: 2. }
    }
}

impl Draw for SeparatorData {
    fn draw(&self, screen: &mut Screen) -> ScreenHandle {
        let response = self.draw_egui(screen.egui_ui);

        ScreenHandle {
            _egui_response: response,
        }
    }
}

impl EguiDraw for SeparatorData {
    fn draw_egui(&self, ui: &mut Ui) -> Response {
        // Reserve space
        let available_space = ui.available_size_before_wrap();
        let height = self.thickness + 10.0;
        let width = available_space.x;
        let requested_size = vec2(width, height);

        let (given_rect, response) = Ui::allocate_at_least(ui, requested_size, Sense::click());

        // Draw  - if it is located in the visible part of the screen
        if ui.is_rect_visible(response.rect) {
            let painter = ui.painter();
            let draw_rect = Rect {
                min: [given_rect.min.x, given_rect.min.y + 10. / 2.].into(),
                max: [given_rect.max.x, given_rect.max.y - 10. / 2.].into(),
            };
            // painter.rect_filled(
            //     given_rect,
            //     Rounding::none(),
            //     Color32::GOLD.gamma_multiply(0.1),
            // );
            painter.rect_filled(draw_rect, Rounding::none(), Color32::GRAY);
        }
        response
    }
}
