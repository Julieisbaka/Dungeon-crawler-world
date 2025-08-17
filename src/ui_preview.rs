use std::collections::HashMap;

use egui::Context;

// Bring in existing UI modules
use crate::new_save::{self, NewSaveState};
use crate::saves::{self, SaveMenuState};
use crate::settings::{self, Settings};
use crate::skills::{self, SkillsState};
use crate::console::{self, ConsoleState};

pub struct UiPreviewManager {
    windows: HashMap<String, PreviewWindow>,
}

impl Default for UiPreviewManager {
    fn default() -> Self { Self { windows: HashMap::new() } }
}

enum PreviewWindow {
    Skills { open: bool, state: SkillsState },
    NewSave { open: bool, state: NewSaveState },
    Saves { open: bool, state: SaveMenuState },
    Settings { open: bool, settings: Settings },
    Console { open: bool, state: ConsoleState },
}

impl UiPreviewManager {
    pub fn new() -> Self { Self::default() }

    pub fn known_names() -> &'static [&'static str] {
        &["skills", "new_save", "saves", "settings", "console"]
    }

    pub fn open_preview(&mut self, name: &str) -> Result<(), String> {
        let key = name.trim().to_lowercase();
        let window = match key.as_str() {
            "skills" => self
                .windows
                .entry(key)
                .or_insert_with(|| PreviewWindow::Skills { open: true, state: SkillsState::default() }),
            "new_save" => self
                .windows
                .entry(key)
                .or_insert_with(|| PreviewWindow::NewSave { open: true, state: NewSaveState::default() }),
            "saves" => self
                .windows
                .entry(key)
                .or_insert_with(|| PreviewWindow::Saves { open: true, state: SaveMenuState::default() }),
            "settings" => self
                .windows
                .entry(key)
                .or_insert_with(|| PreviewWindow::Settings { open: true, settings: Settings::default() }),
            "console" => self
                .windows
                .entry(key)
                .or_insert_with(|| PreviewWindow::Console { open: true, state: ConsoleState::default() }),
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
            | PreviewWindow::Console { open, .. } => {
                *open = true;
            }
        }
        Ok(())
    }

    pub fn render(&mut self, ctx: &Context, dev_enabled: bool) {
        // Render each open preview window
        let mut to_close: Vec<String> = Vec::new();
        for (name, win) in self.windows.iter_mut() {
            match win {
                PreviewWindow::Skills { open, state } => {
                    if !*open { continue; }
                    let mut is_open = true;
                    egui::Window::new("Preview: Skills")
                        .open(&mut is_open)
                        .resizable(true)
                        .vscroll(true)
                        .default_size(egui::vec2(500.0, 350.0))
                        .show(ctx, |ui| {
                            skills::skills_ui(ui, state);
                        });
                    if !is_open { *open = false; }
                }
                PreviewWindow::NewSave { open, state } => {
                    if !*open { continue; }
                    let mut is_open = true;
                    egui::Window::new("Preview: New Save")
                        .open(&mut is_open)
                        .resizable(true)
                        .vscroll(true)
                        .default_size(egui::vec2(480.0, 320.0))
                        .show(ctx, |ui| {
                            let _ = new_save::show_new_save_ui(ui, state);
                        });
                    if !is_open { *open = false; }
                }
                PreviewWindow::Saves { open, state } => {
                    if !*open { continue; }
                    let mut is_open = true;
                    egui::Window::new("Preview: Saves Menu")
                        .open(&mut is_open)
                        .resizable(true)
                        .vscroll(true)
                        .default_size(egui::vec2(560.0, 360.0))
                        .show(ctx, |ui| {
                            saves::show_save_ui(ui, state);
                        });
                    if !is_open { *open = false; }
                }
                PreviewWindow::Settings { open, settings } => {
                    if !*open { continue; }
                    let mut is_open = true;
                    egui::Window::new("Preview: Settings")
                        .open(&mut is_open)
                        .resizable(true)
                        .vscroll(true)
                        .default_size(egui::vec2(420.0, 320.0))
                        .show(ctx, |ui| {
                            settings::settings_ui(ui, settings, dev_enabled);
                        });
                    if !is_open { *open = false; }
                }
                PreviewWindow::Console { open, state } => {
                    if !*open { continue; }
                    let mut is_open = true;
                    egui::Window::new("Preview: Console")
                        .open(&mut is_open)
                        .resizable(true)
                        .vscroll(true)
                        .default_size(egui::vec2(520.0, 300.0))
                        .show(ctx, |ui| {
                            console::console_ui(ui, state);
                        });
                    if !is_open { *open = false; }
                }
            }
            // Mark for cleanup if fully closed
            match win {
                PreviewWindow::Skills { open, .. }
                | PreviewWindow::NewSave { open, .. }
                | PreviewWindow::Saves { open, .. }
                | PreviewWindow::Settings { open, .. }
                | PreviewWindow::Console { open, .. } => {
                    if !*open { to_close.push(name.clone()); }
                }
            }
        }
        for key in to_close { self.windows.remove(&key); }
    }
}
