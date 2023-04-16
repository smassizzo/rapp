use rapp::RustApp;

use crate::run;

struct App {}

impl RustApp for App {
    fn pages(&mut self) -> std::collections::HashMap<String, rapp::PageFn> {
        todo!()
    }

    fn current_page(&self) -> String {
        todo!()
    }

    fn start(&mut self) {
        todo!()
    }

    fn new() -> Self
    where
        Self: Sized,
    {
        todo!()
    }
}

fn test() {
    println!("RUN 2");
    let app = Box::new(App::new());
    let _ = run(app);
}
