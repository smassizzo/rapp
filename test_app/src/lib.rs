use rapp::{Page, PageFn, RustApp};
use std::sync::Arc;

#[derive(Default)]
pub struct App {}

impl RustApp for App {
    fn pages(&mut self) -> Vec<PageFn> {
        let mut pages = vec![];

        let page1: Arc<Box<dyn Page>> = Arc::new(Box::new(HomePage {}));
        pages.push(page1);

        let page2: Arc<Box<dyn Page>> = Arc::new(Box::new(DetailsPage {}));
        pages.push(page2);

        pages
    }

    fn current_page(&self) -> String {
        String::new()
    }

    fn start(&mut self) {}

    fn new() -> Self {
        Self {}
    }
}

struct HomePage;

impl HomePage {}

impl Page for HomePage {
    fn name(&self) -> String {
        "HomePage".to_string()
    }

    fn show(&mut self) {
        println!("{}", self.name());
    }
}

struct DetailsPage;

impl HomePage {}

impl Page for DetailsPage {
    fn name(&self) -> String {
        "DetailsPage".to_string()
    }

    fn show(&mut self) {
        println!("{}", self.name());
    }
}
