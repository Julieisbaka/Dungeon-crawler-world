// Import necessary crates and modules from eframe and egui
use eframe::{App, Frame, NativeOptions};
use egui::{CentralPanel, Context, RichText, Style, Visuals};

// Define your main application struct.
// This struct will hold any state your application needs.
// For a simple menu, it might not need any fields, but it's essential
// for eframe to manage your application's lifecycle.
struct DungeonCrawlerworld;

// Implement the Default trait for DungeonCrawlerworld.
// This allows eframe to create a default instance of your app when it starts.
impl Default for DungeonCrawlerworld {
    fn default() -> Self {
        Self {} // No fields to initialize for this simple menu
    }
}

// Implement the eframe::App trait for your MyApp struct.
// This trait defines the core behavior of your egui application,
// specifically the `update` method which is called repeatedly to redraw the UI.
impl App for DungeonCrawlerworld {
    // The `update` method is where you define your UI layout and logic.
    // `ctx`: The egui context, used to interact with the GUI.
    // `_frame`: The eframe frame, used for window operations (e.g., closing the app).
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        // Create a CentralPanel, which is a good default for the main content area.
        egui::CentralPanel::default().show(ctx, |ui| {
            // Arrange UI elements vertically and centered within the panel.
            ui.vertical_centered(|ui| {
                // Add a heading for the menu.
                // RichText allows for styling like size.
                ui.heading(RichText::new("Game Menu").size(30.0));
                // Add some vertical space for better layout.
                ui.add_space(20.0);

                // Create buttons and handle their clicks.
                // When a button is clicked, its `clicked()` method returns true.
                if ui.button("New Game").clicked() {
                }
                if ui.button("Saves").clicked() {
                    // Handle loading game states.
                }
                if ui.button("Settings").clicked() {
                    // Handle opening settings menu.
                }
                if ui.button("Quit").clicked() {
                    // Request the eframe window to close.
                    _frame.close();
                }
            });
        });
    }
}

// The `main` function is the entry point of your Rust executable.
// It sets up the eframe environment and runs your egui application.
fn main() -> eframe::Result<()> {
    // Define native window options, such as initial size and title.
    let options = NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 300.0]) // Initial window size (width, height)
            .with_min_inner_size([300.0, 200.0]) // Minimum resizable size
            .with_title("Dungeon crawler world"), // Window title
        ..Default::default() // Use default values for other options
    };

    // Run the eframe application.
    // This function takes the application name, options, and a closure
    // that creates and returns your App instance.
    eframe::run_native(
        "Dungeon crawler world", // The name of your application (also used as default window title)
        options,
        Box::new(|creation_context| {
            // This closure is called once when the application starts.
            // It's a good place to set up global egui styles.
            creation_context.egui_ctx.set_style(Style {
                visuals: Visuals::dark(), // Set egui to use its default dark theme
                ..Default::default()
            });
            // Return a boxed instance of your DungeonCrawlerCarl.
            Box::new(DungeonCrawlerCarl::default())
        }),
    )
}