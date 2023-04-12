//type RustApp = test_app::App;
type ThisApp = test_app::App;
use rapp::RustApp;

fn main() {
    let mut app = ThisApp::new();
    app.start();
    let pages = app.pages();
    let _current = app.current_page();
    for p in pages.values() {
        println!("Page '{}'", p.name());
    }
}
