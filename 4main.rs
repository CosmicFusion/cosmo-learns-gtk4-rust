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

    // Create a label called "_warning_label"
    let _warning_label = Label::builder()
        // Label Text
        .label("DO NOT LISTEN TO THE BUTTON!")
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
    
    // Create a button called "_click_me_button"
    let _click_me_button = Button::builder()
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
        
    // Create A box
    let _main_box = gtk::Box::builder()
        // that puts items vertically
        .orientation(Orientation::Vertical)
        .build();
    // Add the "_warning_label" to "_main_box"
    _main_box.append(&_warning_label);
    // Add the "_click_me_button" to "_main_box"
    _main_box.append(&_click_me_button);
        
    // create the main Application window
    let window = ApplicationWindow::builder()
        // The text on the titlebar
        .title("Cosmo Test GTK4 RS")
        // link it to the application "app"
        .application(app)
        // Add the box called "_main_box" to it
        .child(&_main_box)
        // build the window
        .build();
        
    // Connects the clicking of  "_click_me_button" to the external function "print_why" and idk why but everyone tells me to be "move |_| " before the external function
    /// and instead of () we put an aurgment for the target label with & before it so it's"
    /// print_why() -> print_why(&_warning_label)
    _click_me_button.connect_clicked(move |_| print_why(&_warning_label));
        
        
    // show the window
    window.show()
}


// an external function to be called via "_click_me_button.connect_clicked"
fn print_why(label: &Label) {
    // takes the aurgument from "_click_me_button.connect_clicked" which should be a label amd sets its text to "Why would you :("
    label.set_text("Why would you :(");
}
