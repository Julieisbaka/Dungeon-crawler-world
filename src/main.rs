// Import necessary crates and modules from eframe and egui
use eframe::{App, Frame, NativeOptions};
use egui::{CentralPanel, Context, RichText, Style, Visuals};
mod settings;
use settings::{Settings, settings_ui};


// Main app struct with settings state
struct DungeonCrawlerworld {
    show_settings: bool,
    settings: Settings,
}


impl Default for DungeonCrawlerworld {
    fn default() -> Self {
        Self {
            show_settings: false,
            settings: Settings::default(),
        }
    }
}


impl App for DungeonCrawlerworld {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui| {
            if self.show_settings {
                ui.vertical_centered(|ui| {
                    ui.heading(RichText::new("Settings").size(28.0));
                    settings_ui(ui, &mut self.settings);
                    ui.add_space(16.0);
                    if ui.button("Back").clicked() {
                        self.show_settings = false;
                    }
                });
            } else {
                ui.vertical_centered(|ui| {
                    ui.heading(RichText::new("Game Menu").size(30.0));
                    ui.add_space(20.0);
                    if ui.button("New Game").clicked() {}
                    if ui.button("Saves").clicked() {
                        // Handle loading game states.
                    }
                    if ui.button("Settings").clicked() {
                        self.show_settings = true;
                    }
                    if ui.button("Quit").clicked() {
                        frame.close();
                    }
                });
            }
        });
    }
}

// The `main` function is the entry point of your Rust executable.
// It sets up the eframe environment and runs your egui application.
fn main() -> eframe::Result<()> {
    // Define native window options, such as initial size and title.
    let options: NativeOptions = NativeOptions {
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
        Box::new(|creation_context: &eframe::CreationContext<'_>| {
            // This closure is called once when the application starts.
            // It's a good place to set up global egui styles.
            creation_context.egui_ctx.set_style(Style {
                visuals: Visuals::dark(), // Set egui to use its default dark theme
                ..Default::default()
            });
            // Return a boxed instance of your DungeonCrawlerworld.
            Box::new(DungeonCrawlerworld::default())
        }),
    )
}
