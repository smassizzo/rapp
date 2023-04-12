use std::{collections::HashMap, sync::Arc};

use rapp::{Page, PageFn, RustApp};

#[derive(Default)]
pub struct App {}

impl RustApp for App {
    fn pages(&mut self) -> HashMap<String, PageFn> {
        let mut map: HashMap<String, PageFn> = HashMap::new();

        let page1: Arc<Box<dyn Page>> = Arc::new(Box::new(HomePage {}));
        map.insert(page1.name(), page1);

        let page2: Arc<Box<dyn Page>> = Arc::new(Box::new(DetailsPage {}));
        map.insert(page2.name(), page2);

        map
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
