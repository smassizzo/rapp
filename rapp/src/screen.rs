use crate::drawables::{placeholder::PlaceHolder, separator::Separator, traits::Draw};
use egui::{Response, Ui};
use std::{cell::RefCell, rc::Rc, sync::Arc};

pub struct Screen<'a> {
    pub egui_ui: &'a mut Ui,
    pub queue: RefCell<Vec<Rc<RefCell<dyn Draw>>>>,
}

impl Screen<'_> {
    pub fn draw(&mut self) {
        let mut other = vec![];
        {
            let len = self.queue.borrow().len();
            let mut a = self.queue.borrow_mut();
            let bs = a.drain(0..len);
            for x in bs {
                other.push(x);
            }
        }
        for b in other {
            b.borrow().draw(self);
        }
    }
}

pub struct ScreenWithArc<'a> {
    pub arc_ui: Arc<&'a mut Ui>,
}

// Separator
impl Screen<'_> {
    pub fn separator(&mut self) -> Separator {
        let default = Separator::default();
        self.queue.borrow_mut().push(default.0.clone());
        default
    }
}

// Placeholder
impl Screen<'_> {
    pub fn placeholder(&mut self) -> PlaceHolder {
        let default = PlaceHolder::default();
        self.queue.borrow_mut().push(default.0.clone());
        default
    }
}

pub struct ScreenHandle {
    pub(crate) _egui_response: Response,
}
