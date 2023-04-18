use super::traits::{Draw, EguiDraw};
use crate::screen::{Screen, ScreenHandle};
use egui::{vec2, Color32, Rect, Response, Rounding, Sense, Ui};
use std::{cell::RefCell, rc::Rc};

#[derive(Clone, Default)]
pub struct PlaceHolder(pub(crate) Rc<RefCell<PlaceHolderData>>);

impl PlaceHolder {
    pub fn width(self, width: u16) -> Self {
        self.0.borrow_mut().width = width.into();
        self
    }
    pub fn height(self, heigth: u16) -> Self {
        self.0.borrow_mut().heigth = heigth.into();
        self
    }
}

#[derive(Clone)]
pub struct PlaceHolderData {
    pub width: f32,
    pub heigth: f32,
}

impl Default for PlaceHolderData {
    fn default() -> Self {
        Self {
            width: 100.,
            heigth: 100.,
        }
    }
}

impl Draw for PlaceHolderData {
    fn draw(&self, screen: &mut Screen) -> ScreenHandle {
        let response = self.draw_egui(screen.egui_ui);

        ScreenHandle {
            _egui_response: response,
        }
    }
}

impl EguiDraw for PlaceHolderData {
    fn draw_egui(&self, ui: &mut Ui) -> Response {
        // Reserve space
        let available_space = ui.available_size_before_wrap();
        let height = if self.heigth == f32::INFINITY {
            available_space.y
        } else {
            self.heigth
        };
        let width = if self.width == f32::INFINITY {
            available_space.x
        } else {
            self.width
        };
        let nest_size = vec2(width, height);
        let (reserved_rect, response) = Ui::allocate_at_least(ui, nest_size, Sense::click());

        // Draw  - if it is located in the visible part of the screen
        if ui.is_rect_visible(response.rect) {
            let painter = ui.painter();
            let draw_rect = Rect {
                min: reserved_rect.min,
                max: [reserved_rect.min.x + width, reserved_rect.max.y].into(),
            };
            painter.rect_filled(reserved_rect, Rounding::none(), Color32::GOLD);
            painter.rect_filled(draw_rect, Rounding::none(), Color32::GREEN);
        }
        response
    }
}
