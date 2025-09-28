use std::collections::HashMap;

use egui::Context;

// Bring in existing UI modules
use crate::console::{self, ConsoleState};
use crate::fps::FpsGraph;
use crate::new_save::{self, NewSaveState};
use crate::saves::{self, SaveMenuState};
use crate::settings::{self, Settings};
use crate::skills::{self, SkillsState};

pub struct UiPreviewManager {
    windows: HashMap<String, PreviewWindow>,
}

impl Default for UiPreviewManager {
    /// Returns a new `UiPreviewManager` with no preview windows initialized.
    ///
    /// # Returns
    /// A new `UiPreviewManager` instance with an empty window registry.
    fn default() -> Self {
        Self {
            windows: HashMap::new(),
        }
    }
}

enum PreviewWindow {
    Skills {
        open: bool,
        max: bool,
        state: SkillsState,
    },
    NewSave {
        open: bool,
        max: bool,
        state: NewSaveState,
    },
    Saves {
        open: bool,
        max: bool,
        state: SaveMenuState,
    },
    Settings {
        open: bool,
        max: bool,
        settings: Settings,
    },
    Console {
        open: bool,
        max: bool,
        state: ConsoleState,
    },
    FpsGraph {
        open: bool,
        max: bool,
        graph: FpsGraph,
    },
    Quit {
        open: bool,
        max: bool,
    },
}

impl UiPreviewManager {
    /// Constructs a new `UiPreviewManager` using the default implementation.
    ///
    /// # Returns
    /// A new `UiPreviewManager` instance.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns a static list of all known preview window names that can be opened.
    ///
    /// # Returns
    /// A static slice of string names for all supported preview windows.
    pub fn known_names() -> &'static [&'static str] {
        &[
            "skills",
            "new_save",
            "saves",
            "settings",
            "console",
            "fps_graph",
            "quit",
        ]
    }

    /// Opens a preview window by name, creating it if it does not already exist.
    ///
    /// # Arguments
    /// * `name` - The name of the preview window to open (case-insensitive).
    ///
    /// # Returns
    /// * `Ok(())` if the window was opened or already exists.
    /// * `Err(String)` if the name is not recognized.
    pub fn open_preview(&mut self, name: &str) -> Result<(), String> {
        let key: String = name.trim().to_lowercase();
        let window: &mut PreviewWindow = match (&key).as_str() {
            "quit" => (&mut (*self).windows)
                .entry(key)
                .or_insert_with(|| -> PreviewWindow {
                    PreviewWindow::Quit {
                        open: true,
                        max: false,
                    }
                }),
            "fps_graph" => (&mut (*self).windows)
                .entry(key)
                .or_insert_with(|| -> PreviewWindow {
                    PreviewWindow::FpsGraph {
                        open: true,
                        max: false,
                        graph: FpsGraph::default(),
                    }
                }),
            "skills" => (&mut (*self).windows)
                .entry(key)
                .or_insert_with(|| -> PreviewWindow {
                    let mut st: SkillsState = SkillsState::default();
                    // In preview, show all discovered skills only when dev-mode is enabled
                    // and enable developer controls conditionally.
                    if cfg!(feature = "dev-mode") {
                        (&mut st).enable_preview();
                        (&mut st).enable_dev_controls();
                    }
                    PreviewWindow::Skills {
                        open: true,
                        max: false,
                        state: st,
                    }
                }),
            "new_save" => (&mut (*self).windows)
                .entry(key)
                .or_insert_with(|| -> PreviewWindow {
                    PreviewWindow::NewSave {
                        open: true,
                        max: false,
                        state: NewSaveState::default(),
                    }
                }),
            "saves" => (&mut (*self).windows)
                .entry(key)
                .or_insert_with(|| -> PreviewWindow {
                    PreviewWindow::Saves {
                        open: true,
                        max: false,
                        state: SaveMenuState::default(),
                    }
                }),
            "settings" => (&mut (*self).windows)
                .entry(key)
                .or_insert_with(|| -> PreviewWindow {
                    PreviewWindow::Settings {
                        open: true,
                        max: false,
                        settings: Settings::default(),
                    }
                }),
            "console" => (&mut (*self).windows)
                .entry(key)
                .or_insert_with(|| -> PreviewWindow {
                    PreviewWindow::Console {
                        open: true,
                        max: false,
                        state: ConsoleState::default(),
                    }
                }),
            other => {
                return Err(format!(
                    "Unknown UI '{}'. Known: {}",
                    other,
                    Self::known_names().join(", ")
                ))
            }
        };
        // Ensure it's marked open if it already existed
        match window {
            PreviewWindow::Skills { open, .. }
            | PreviewWindow::NewSave { open, .. }
            | PreviewWindow::Saves { open, .. }
            | PreviewWindow::Settings { open, .. }
            | PreviewWindow::Console { open, .. }
            | PreviewWindow::FpsGraph { open, .. }
            | PreviewWindow::Quit { open, .. } => {
                *open = true;
            }
        }
        Ok(())
    }

    /// Renders all open preview windows and removes any that are closed.
    ///
    /// # Arguments
    /// * `ctx` - The egui context to use for rendering.
    /// * `dev_enabled` - If true, developer options are enabled in some windows.
    pub fn render(&mut self, ctx: &Context, dev_enabled: bool) {
        // Render each open preview window
        let mut to_close: Vec<String> = Vec::new();
        let screen: egui::Rect = ctx.screen_rect();
        let screen_size: egui::Vec2 = (&screen).size();
        for (name, win) in (&mut (*self).windows).iter_mut() {
            match win {
                PreviewWindow::Quit { open, max } => {
                    if !*open {
                        continue;
                    }
                    let mut is_open: bool = true;
                    let mut close_after: bool = false;
                    let id: egui::Id = egui::Id::new(("preview_quit", *max));
                    egui::Window::new("Preview: Quit Confirmation")
                        .id(id)
                        .open(&mut is_open)
                        .collapsible(false)
                        .resizable(false)
                        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                        .show(ctx, |ui: &mut egui::Ui| {
                            ui.label("Are you sure you want to quit?");
                            ui.horizontal(|ui: &mut egui::Ui| {
                                if (&ui.button("Yes")).clicked() {
                                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                                }
                                if (&ui.button("No")).clicked() {
                                    close_after = true;
                                }
                            });
                        });
                    if !is_open || close_after {
                        *open = false;
                    }
                }
                PreviewWindow::FpsGraph { open, max, graph } => {
                    if !*open {
                        continue;
                    }
                    let mut is_open: bool = true;
                    let id: egui::Id = egui::Id::new(("preview_fps_graph", *max));
                    egui::Window::new("Preview: FPS Graph")
                        .id(id)
                        .open(&mut is_open)
                        .resizable(true)
                        .vscroll(true)
                        .default_size(egui::vec2(
                            if *max { screen_size.x } else { 300.0 },
                            if *max { screen_size.y } else { 120.0 },
                        ))
                        .max_size(screen_size)
                        .show(ctx, |ui: &mut egui::Ui| {
                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::TOP),
                                |ui: &mut egui::Ui| {
                                    let label: &str = if *max { "Restore" } else { "Maximize" };
                                    if (&ui.button(label)).clicked() {
                                        *max = !*max;
                                    }
                                },
                            );
                            graph.ui(ui);
                        });
                    if !is_open {
                        *open = false;
                    }
                }
                PreviewWindow::Skills { open, max, state } => {
                    if !*open {
                        continue;
                    }
                    let mut is_open: bool = true;
                    let id: egui::Id = egui::Id::new(("preview_skills", *max));
                    egui::Window::new("Preview: Skills")
                        .id(id)
                        .open(&mut is_open)
                        .resizable(true)
                        .vscroll(true)
                        .default_size(egui::vec2(
                            if *max {
                                screen_size.x
                            } else {
                                screen_size.x * 0.9
                            },
                            if *max {
                                screen_size.y
                            } else {
                                (screen_size.y * 0.9).max(500.0)
                            },
                        ))
                        .max_size(screen_size)
                        .show(ctx, |ui: &mut egui::Ui| {
                            // Toolbar
                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::TOP),
                                |ui: &mut egui::Ui| {
                                    let label: &str = if *max { "Restore" } else { "Maximize" };
                                    if (&ui.button(label)).clicked() {
                                        *max = !*max;
                                    }
                                },
                            );
                            skills::skills_ui(ui, state);
                        });
                    if !is_open {
                        *open = false;
                    }
                }
                PreviewWindow::NewSave { open, max, state } => {
                    if !*open {
                        continue;
                    }
                    let mut is_open: bool = true;
                    let id: egui::Id = egui::Id::new(("preview_new_save", *max));
                    egui::Window::new("Preview: New Save")
                        .id(id)
                        .open(&mut is_open)
                        .resizable(true)
                        .vscroll(true)
                        .default_size(egui::vec2(
                            if *max {
                                screen_size.x
                            } else {
                                screen_size.x * 0.9
                            },
                            if *max {
                                screen_size.y
                            } else {
                                screen_size.y * 0.9
                            },
                        ))
                        .max_size(screen_size)
                        .show(ctx, |ui: &mut egui::Ui| {
                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::TOP),
                                |ui: &mut egui::Ui| {
                                    let label: &str = if *max { "Restore" } else { "Maximize" };
                                    if (&ui.button(label)).clicked() {
                                        *max = !*max;
                                    }
                                },
                            );
                            let _ = new_save::show_new_save_ui(ui, state);
                        });
                    if !is_open {
                        *open = false;
                    }
                }
                PreviewWindow::Saves { open, max, state } => {
                    if !*open {
                        continue;
                    }
                    let mut is_open: bool = true;
                    let id: egui::Id = egui::Id::new(("preview_saves", *max));
                    egui::Window::new("Preview: Saves Menu")
                        .id(id)
                        .open(&mut is_open)
                        .resizable(true)
                        .vscroll(true)
                        .default_size(egui::vec2(
                            if *max {
                                screen_size.x
                            } else {
                                screen_size.x * 0.9
                            },
                            if *max {
                                screen_size.y
                            } else {
                                screen_size.y * 0.9
                            },
                        ))
                        .max_size(screen_size)
                        .show(ctx, |ui: &mut egui::Ui| {
                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::TOP),
                                |ui: &mut egui::Ui| {
                                    let label: &str = if *max { "Restore" } else { "Maximize" };
                                    if (&ui.button(label)).clicked() {
                                        *max = !*max;
                                    }
                                },
                            );
                            saves::show_save_ui(ui, state);
                        });
                    if !is_open {
                        *open = false;
                    }
                }
                PreviewWindow::Settings {
                    open,
                    max,
                    settings,
                } => {
                    if !*open {
                        continue;
                    }
                    let mut is_open: bool = true;
                    let id: egui::Id = egui::Id::new(("preview_settings", *max));
                    egui::Window::new("Preview: Settings")
                        .id(id)
                        .open(&mut is_open)
                        .resizable(true)
                        .vscroll(true)
                        .default_size(egui::vec2(
                            if *max {
                                screen_size.x
                            } else {
                                screen_size.x * 0.9
                            },
                            if *max {
                                screen_size.y
                            } else {
                                screen_size.y * 0.9
                            },
                        ))
                        .max_size(screen_size)
                        .show(ctx, |ui: &mut egui::Ui| {
                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::TOP),
                                |ui: &mut egui::Ui| {
                                    let label: &str = if *max { "Restore" } else { "Maximize" };
                                    if (&ui.button(label)).clicked() {
                                        *max = !*max;
                                    }
                                },
                            );
                            settings::settings_ui(ui, settings, dev_enabled);
                        });
                    if !is_open {
                        *open = false;
                    }
                }
                PreviewWindow::Console { open, max, state } => {
                    if !*open {
                        continue;
                    }
                    let mut is_open: bool = true;
                    let id: egui::Id = egui::Id::new(("preview_console", *max));
                    egui::Window::new("Preview: Console")
                        .id(id)
                        .open(&mut is_open)
                        .resizable(true)
                        .default_size(egui::vec2(
                            if *max {
                                screen_size.x
                            } else {
                                screen_size.x * 0.9
                            },
                            if *max {
                                screen_size.y
                            } else {
                                screen_size.y * 0.5
                            },
                        ))
                        .max_size(screen_size)
                        .show(ctx, |ui: &mut egui::Ui| {
                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::TOP),
                                |ui: &mut egui::Ui| {
                                    let label: &str = if *max { "Restore" } else { "Maximize" };
                                    if (&ui.button(label)).clicked() {
                                        *max = !*max;
                                    }
                                },
                            );
                            console::console_ui(ui, state, 300);
                        });
                    if !is_open {
                        *open = false;
                    }
                }
            }
            // Mark for cleanup if fully closed
            match win {
                PreviewWindow::Skills { open, .. }
                | PreviewWindow::NewSave { open, .. }
                | PreviewWindow::Saves { open, .. }
                | PreviewWindow::Settings { open, .. }
                | PreviewWindow::Console { open, .. }
                | PreviewWindow::FpsGraph { open, .. }
                | PreviewWindow::Quit { open, .. } => {
                    if !*open {
                        (&mut to_close).push(name.clone());
                    }
                }
            }
        }
        for key in to_close {
            (&mut (*self).windows).remove(&key);
        }
    }
}
