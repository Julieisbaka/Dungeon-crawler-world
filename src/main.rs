use std::sync::Mutex;
use once_cell::sync::Lazy;

// Global current_save variable
pub static CURRENT_SAVE: Lazy<Mutex<Option<String>>> = Lazy::new(|| -> Mutex<Option<String>> { Mutex::new(None) });

pub fn set_current_save(save_name: &str) {
    let mut current: std::sync::MutexGuard<'_, Option<String>> = CURRENT_SAVE.lock().unwrap();
    *current = Some(save_name.to_string());
    log::info!("Current save set to: {}", save_name);
}
use std::error::Error;

// Import necessary crates and modules from eframe and egui
use eframe::{App, Frame, NativeOptions};
use egui::{CentralPanel, Context, RichText, Style, Visuals};
mod saves;
use saves::show_save_ui;
mod new_save;
mod settings;
use settings::{settings_ui, Settings};
mod console;
use console::{console_ui, ConsoleState};
mod skills;
mod ui_preview;
use ui_preview::UiPreviewManager;

// Developer mode flag is controlled via Cargo feature `dev-mode`.
// Enabled in debug builds by default via Cargo.toml [features].
// For release builds in CI, we pass --no-default-features to disable it.
const DEV_MODE_ENABLED: bool = cfg!(feature = "dev-mode");

// Main app struct with settings state
struct DungeonCrawlerworld {
    show_settings: bool,
    show_saves: bool,
    settings: Settings,
    save_menu_state: saves::SaveMenuState,
    console_state: ConsoleState,
    ui_preview: UiPreviewManager,
}


impl Default for DungeonCrawlerworld {
    fn default() -> Self {
        Self {
            show_settings: false,
            show_saves: false,
            settings: Settings::default(),
            save_menu_state: saves::SaveMenuState::default(),
            console_state: ConsoleState::default(),
            ui_preview: UiPreviewManager::new(),
        }
    }
}

impl App for DungeonCrawlerworld {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui| {
            if (*self).show_settings {
                ui.heading(RichText::new("Settings").size(28.0));
                settings_ui(ui, &mut self.settings, DEV_MODE_ENABLED);
                ui.add_space(16.0);
                if ui.button("Back").clicked() {
                    (*self).show_settings = false;
                }
            } else if (*self).show_saves {
                ui.heading(RichText::new("Saves Menu").size(28.0));
                show_save_ui(ui, &mut (*self).save_menu_state);
                ui.add_space(16.0);
                if ui.button("Back").clicked() {
                    (*self).show_saves = false;
                }
            } else {
                ui.heading(RichText::new("Game Menu").size(30.0));
                ui.add_space(20.0);
                if ui.button("Saves").clicked() {
                    (*self).show_saves = true;
                }
                if ui.button("Settings").clicked() {
                    (*self).show_settings = true;
                }
                if ui.button("Quit").clicked() {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }
            }
        });

        // Developer Console window: available only when dev feature is enabled.
        if DEV_MODE_ENABLED && self.settings.developer_mode && self.settings.show_console {
            let mut open = true;
            egui::Window::new("Console")
                .open(&mut open)
                .resizable(true)
                .vscroll(true)
                .hscroll(false)
                .default_size(egui::vec2(500.0, 250.0))
                .show(ctx, |ui| {
                    // Intercept invoke commands to open previews
                    // Render console UI first
                    console_ui(ui, &mut self.console_state);
                    // Provide a minimal inline help mention for invoke
                    // (kept non-intrusive in UI; full help prints in console)
                });
            if !open {
                // Closing the window hides the console until re-enabled in settings
                self.settings.show_console = false;
                self.settings.save();
            }
            // After UI event handling, process any queued commands
            for cmd in self.console_state.take_pending() {
                let trimmed = cmd.trim();
                if trimmed.is_empty() { continue; }
                let mut parts = trimmed.split_whitespace();
                let head = parts.next().unwrap_or("");
                match head {
                    "invoke" => {
                        let name = parts.collect::<Vec<_>>().join(" ");
                        if name.is_empty() {
                            self.console_state.log_line("Usage: invoke <ui>");
                        } else {
                            if DEV_MODE_ENABLED && self.settings.developer_mode {
                                match self.ui_preview.open_preview(&name) {
                                    Ok(()) => self.console_state.log_line(format!("Invoked UI preview: {}", name)),
                                    Err(e) => self.console_state.log_line(e),
                                }
                            } else {
                                self.console_state.log_line("UI previews are only available in Developer Mode.");
                            }
                        }
                    }
                    // Fallback to built-in commands
                    _ => self.console_state.run_command(trimmed),
                }
            }
        }

        // Render any active preview windows (gated by dev mode so previews are a dev tool)
        if DEV_MODE_ENABLED && self.settings.developer_mode {
            self.ui_preview.render(ctx, DEV_MODE_ENABLED);
        }
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
