#![cfg_attr(all(windows, not(debug_assertions)), windows_subsystem = "windows")]

use once_cell::sync::Lazy;
use std::sync::Mutex;

// Global current_save variable
pub static CURRENT_SAVE: Lazy<Mutex<Option<String>>> =
    Lazy::new(|| -> Mutex<Option<String>> { Mutex::new(None) });

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
use settings::{settings_ui, Settings, SettingsResult};
mod console;
use console::{console_ui, ConsoleState};
mod skills;
mod ui_preview;
use ui_preview::UiPreviewManager;
mod fps;
use fps::FpsGraph;

/// Developer mode flag is controlled via Cargo feature `dev-mode`.
/// Enabled in debug builds by default via Cargo.toml [features].
/// For release builds in CI, we pass --no-default-features to disable it.
const DEV_MODE_ENABLED: bool = cfg!(feature = "dev-mode");

/// Main app struct with settings state
struct DungeonCrawlerworld {
    show_settings: bool,
    show_saves: bool,
    settings: Settings,
    save_menu_state: saves::SaveMenuState,
    console_state: ConsoleState,
    ui_preview: UiPreviewManager,
    last_fullscreen: Option<bool>,
    fps: FpsGraph,
    // Console session control
    console_open: bool,
    last_show_console: bool,
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
            last_fullscreen: None,
            fps: FpsGraph::default(),
            console_open: false,
            last_show_console: Settings::default().show_console,
        }
    }
}

impl App for DungeonCrawlerworld {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        // Always repaint so the FPS graph and other time-based UI update in real time
        ctx.request_repaint();
        // Apply fullscreen setting when it changes
        if (*self).last_fullscreen != Some((*self).settings.fullscreen) {
            (*self).last_fullscreen = Some((*self).settings.fullscreen);
            ctx.send_viewport_cmd(egui::ViewportCommand::Fullscreen(
                (*self).settings.fullscreen,
            ));
        }

        // Update FPS graph with delta time in ms
        let dt_ms: f32 = ctx.input(|i: &egui::InputState| (*i).stable_dt) * 1000.0;
        (*self).fps.push_frame_time(dt_ms);

        CentralPanel::default()
            .frame(
                egui::Frame::central_panel(&ctx.style())
                    .inner_margin(egui::Margin::same(0))
                    .outer_margin(egui::Margin::same(0)),
            )
            .show(ctx, |ui: &mut egui::Ui| {
                // Allocate a full-screen area and center content within it
                let avail: egui::Vec2 = ui.available_size();
                ui.allocate_ui_with_layout(
                    avail,
                    egui::Layout::top_down(egui::Align::Center),
                    |ui: &mut egui::Ui| {
                        if (*self).show_settings {
                            ui.heading(RichText::new("Settings").size(28.0));
                            ui.add_space(8.0);
                            let mut back: bool = false;
                            egui::ScrollArea::vertical().auto_shrink([false; 2]).show(
                                ui,
                                |ui: &mut egui::Ui| {
                                    // Constrain a readable max width while still centered in the full area
                                    ui.set_max_width(700.0);
                                    let res: SettingsResult =
                                        settings_ui(ui, &mut (*self).settings, DEV_MODE_ENABLED);
                                    if res.request_save {
                                        (*self).settings.save();
                                    }
                                    if res.request_back {
                                        back = true;
                                    }
                                },
                            );
                            if back {
                                (*self).show_settings = false;
                            }
                        } else if (*self).show_saves {
                            ui.heading(RichText::new("Saves Menu").size(28.0));
                            ui.add_space(8.0);
                            egui::ScrollArea::vertical().auto_shrink([false; 2]).show(
                                ui,
                                |ui: &mut egui::Ui| {
                                    ui.set_max_width(900.0);
                                    show_save_ui(ui, &mut (*self).save_menu_state);
                                },
                            );
                            // Respect back request from saves UI
                            if (*self).save_menu_state.back_requested {
                                (*self).save_menu_state.back_requested = false;
                                (*self).show_saves = false;
                            }
                        } else {
                            // Game Menu centered with larger controls
                            ui.add_space(8.0);
                            ui.heading(RichText::new("Game Menu").size(30.0));
                            ui.add_space(24.0);
                            // Slightly larger buttons for presence
                            if ui
                                .add_sized([220.0, 36.0], egui::Button::new("Saves"))
                                .clicked()
                            {
                                (*self).show_saves = true;
                            }
                            ui.add_space(8.0);
                            if ui
                                .add_sized([220.0, 36.0], egui::Button::new("Settings"))
                                .clicked()
                            {
                                (*self).show_settings = true;
                            }
                            ui.add_space(8.0);
                            if ui
                                .add_sized([220.0, 36.0], egui::Button::new("Quit"))
                                .clicked()
                            {
                                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                            }
                        }
                    },
                );
            });

        // Developer Console window: only when enabled and explicitly opened this session
        // Detect setting edge to open on user toggle (not on startup load)
        if (*self).settings.show_console != (*self).last_show_console {
            if (*self).settings.show_console {
                (*self).console_open = true;
            }
            (*self).last_show_console = (*self).settings.show_console;
        }

        if DEV_MODE_ENABLED && (*self).settings.developer_mode && (*self).console_open {
            let mut open: bool = true;
            egui::Window::new("Console")
                .open(&mut open)
                .resizable(true)
                .vscroll(true)
                .hscroll(false)
                .default_size(egui::vec2(500.0, 250.0))
                .show(ctx, |ui| {
                    // Intercept invoke commands to open previews
                    // Render console UI first
                    console_ui(ui, &mut (*self).console_state);
                    // Provide a minimal inline help mention for invoke
                    // (kept non-intrusive in UI; full help prints in console)
                });
            if !open {
                // Closing the window hides the console until re-enabled in settings
                (*self).settings.show_console = false;
                (*self).console_open = false;
                (*self).settings.save();
            }
            // After UI event handling, process any queued commands
            for cmd in (*self).console_state.take_pending() {
                let trimmed = cmd.trim();
                if trimmed.is_empty() {
                    continue;
                }
                let mut parts: std::str::SplitWhitespace<'_> = trimmed.split_whitespace();
                let head = parts.next().unwrap_or("");
                match head {
                    "invoke" => {
                        let name: String = parts.collect::<Vec<_>>().join(" ");
                        if name.is_empty() {
                            (*self).console_state.log_line("Usage: invoke <ui>");
                        } else {
                            if DEV_MODE_ENABLED && (*self).settings.developer_mode {
                                match (*self).ui_preview.open_preview(&name) {
                                    Ok(()) => (*self)
                                        .console_state
                                        .log_line(format!("Invoked UI preview: {}", name)),
                                    Err(e) => (*self).console_state.log_line(e),
                                }
                            } else {
                                (*self)
                                    .console_state
                                    .log_line("UI previews are only available in Developer Mode.");
                            }
                        }
                    }
                    // Fallback to built-in commands
                    _ => (*self).console_state.run_command(trimmed),
                }
            }
        }

        // Render any active preview windows (gated by dev mode so previews are a dev tool)
        if DEV_MODE_ENABLED && (*self).settings.developer_mode {
            (*self).ui_preview.render(ctx, DEV_MODE_ENABLED);

            // FPS graph overlay in the bottom-right corner when enabled
            if (*self).settings.show_fps_graph {
                egui::TopBottomPanel::bottom("fps_graph_panel")
                    .resizable(false)
                    .min_height(90.0)
                    .show_separator_line(false)
                    .show(ctx, |ui: &mut egui::Ui| {
                        ui.with_layout(
                            egui::Layout::right_to_left(egui::Align::Min),
                            |ui: &mut egui::Ui| {
                                (*self).fps.ui(ui);
                            },
                        );
                    });
            }
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
