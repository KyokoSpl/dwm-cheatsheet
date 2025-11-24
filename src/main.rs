mod keybindings;
mod ui;

use eframe::egui;
use ui::CheatsheetApp;

fn main() -> Result<(), eframe::Error> {
    env_logger::init();
    
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_title("DWM Keybinding Cheatsheet")
            .with_min_inner_size([800.0, 600.0]),
        ..Default::default()
    };
    
    eframe::run_native(
        "DWM Cheatsheet",
        options,
        Box::new(|cc| {
            // Configure custom fonts and styles here if needed
            Ok(Box::new(CheatsheetApp::new(cc)))
        }),
    )
}
