mod keybindings;
mod ui;

use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow};
use gtk4 as gtk;
use gtk4::glib::Propagation;

const APP_ID: &str = "com.dwm.cheatsheet";

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);
    
    // Handle application shutdown
    app.connect_shutdown(|_| {
        println!("Application is shutting down");
    });

    app.run()
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("DWM Keybinding Cheatsheet")
        .default_width(1100)
        .default_height(900)
        .build();

    // Make sure window can receive key events
    window.set_can_focus(true);
    window.set_focusable(true);

    let main_widget = ui::build_main_widget();
    window.set_child(Some(&main_widget));
    
    let app_clone = app.clone();
    let key_controller = gtk::EventControllerKey::new();
    key_controller.connect_key_pressed(move |_, key, _, _| {
        println!("Key pressed: {:?}", key);
        if key == gtk::gdk::Key::Escape {
            println!("Escape key pressed, quitting application");
            app_clone.quit();
            Propagation::Stop
        } else {
            Propagation::Proceed
        }
    });
    
    // Make sure the key controller is set up properly
    key_controller.set_propagation_phase(gtk4::PropagationPhase::Capture);
    window.add_controller(key_controller);

    window.present();
    
    // Try to grab focus
    window.grab_focus();
}
