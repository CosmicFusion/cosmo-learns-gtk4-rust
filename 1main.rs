use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};

fn main() {
    let app = Application::builder()
        .application_id("com.github.gtk4-rs-test.cosmo")
        .build();
        
    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .title("Cosmo Test GTK4 RS")
        .application(app)
        .build();
        
    window.show()
}
