// Use libraries
/// Use all gtk4 libraries (gtk4 -> gtk because cargo)
use gtk::prelude::*;
use gtk::*;


/// main function
fn main() {
    // defining app
    let app = Application::builder()
        // app id
        .application_id("com.github.gtk4-rs-test.cosmo")
        // build app
        .build();
        
    // link the start up of the app with the build_ui function down below
    app.connect_activate(build_ui);
    // start up the app
    app.run();
}


// build ui function linked to app startup above
fn build_ui(app: &Application) {
    // Create a button called "_click_me_button"
    let _click_me_button =   Button::builder()
        // Button Label
        .label("Click ME")
        // Add Space to the top
        .margin_top(12)
        // Add Space to the buttom
        .margin_bottom(12)
        // Add Space to the left
        .margin_start(12)
        // Add Space to the right
        .margin_end(12)
        // build the button
        .build();
    // create the main Application window
    let window = ApplicationWindow::builder()
        // The text on the titlebar
        .title("Cosmo Test GTK4 RS")
        // link it to the application "app"
        .application(app)
        // Add the button called "_click_me_button" to it
        .child(&_click_me_button)
        // build the window
        .build();
        
        
    // show the window
    window.show()
}
