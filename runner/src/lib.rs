use rapp::RustApp;

pub fn run(mut app: impl RustApp) {
    app.start();
    let pages = app.pages();
    let _current = app.current_page();
    for p in pages.values() {
        println!("Page '{}'", p.name());
    }
}
