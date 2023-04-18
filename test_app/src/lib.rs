use rapp::{screen::Screen, Page, PageFn, RustApp};

#[derive(Default)]
pub struct App {}

impl RustApp for App {
    fn pages(&mut self) -> Vec<PageFn> {
        let mut pages = vec![];

        let page1: PageFn = Box::new(HomePage {});
        pages.push(page1);

        let page2: PageFn = Box::new(DetailsPage {});
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

impl Page for HomePage {
    fn name(&self) -> String {
        "HomePage".to_string()
    }

    fn show(&mut self, screen: &mut Screen) {
        screen.placeholder().height(150);
    }
}

struct DetailsPage;

impl Page for DetailsPage {
    fn name(&self) -> String {
        "DetailsPage".to_string()
    }

    fn show(&mut self, screen: &mut Screen) {
        screen.placeholder().height(20).width(70);
        screen.placeholder();

        screen.separator().thickness(10);
        screen.separator();
        screen.separator().thickness(1).thickness(3).thickness(5);
    }
}
