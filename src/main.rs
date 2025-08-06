use std::error::Error;

// Import necessary crates and modules from eframe and egui
use eframe::{App, Frame, NativeOptions};
use egui::{CentralPanel, Context, RichText, Style, Visuals};
mod saves;
use saves::show_save_ui;
mod settings;
use settings::{Settings, settings_ui};


// Main app struct with settings state
struct DungeonCrawlerworld {
    show_settings: bool,
    show_saves: bool,
    settings: Settings,
    save_menu_state: saves::SaveMenuState,
}


impl Default for DungeonCrawlerworld {
    fn default() -> Self {
        Self {
            show_settings: false,
            show_saves: false,
            settings: Settings::default(),
            save_menu_state: saves::SaveMenuState::default(),
        }
    }
}

impl App for DungeonCrawlerworld {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui| {
            if (*self).show_settings {
                ui.vertical_centered(|ui| {
                    ui.heading(RichText::new("Settings").size(28.0));
                    settings_ui(ui, &mut (*self).settings);
                    ui.add_space(16.0);
                    if ui.button("Back").clicked() {
                        (*self).show_settings = false;
                    }
                });
            } else if (*self).show_saves {
                ui.vertical_centered(|ui: &mut egui::Ui| {
                    ui.heading(RichText::new("Saves Menu").size(28.0));
                    show_save_ui(ui, &mut (*self).save_menu_state);
                    ui.add_space(16.0);
                    if ui.button("Back").clicked() {
                        (*self).show_saves = false;
                    }
                });
            } else {
                ui.vertical_centered(|ui| {
                    ui.heading(RichText::new("Game Menu").size(30.0));
                    ui.add_space(20.0);
                    if ui.button("New Game").clicked() {}
                    if ui.button("Saves").clicked() {
                        (*self).show_saves = true;
                    }
                    if ui.button("Settings").clicked() {
                        self.show_settings = true;
                    }
                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
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
        Box::new(|creation_context: &eframe::CreationContext<'_>| -> Result<Box<dyn App>, Box<dyn Error + Send + Sync>> {
            // This closure is called once when the application starts.
            // It's a good place to set up global egui styles.
            (*creation_context).egui_ctx.set_style(Style {
                visuals: Visuals::dark(), // Set egui to use its default dark theme
                ..Default::default()
            });
            // Return a boxed instance of your DungeonCrawlerworld.
            Ok(Box::new(DungeonCrawlerworld::default()))
        }),
    )
}
