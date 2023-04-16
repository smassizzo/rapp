use rapp::RustApp;

fn main() {
    // new
    let app = app::App::new();

    // box
    let app = Box::new(app);

    // run
    let _ = runner::run(app);
}
