use std::{collections::HashMap, sync::Arc};

pub type PageFn = Arc<Box<dyn Page>>;

pub trait RustApp {
    fn pages(&mut self) -> HashMap<String, PageFn>;

    fn current_page(&self) -> String;

    fn start(&mut self);

    fn new() -> Self
    where
        Self: Sized;
}

pub trait Page {
    fn name(&self) -> String;
    fn show(&mut self);
}
