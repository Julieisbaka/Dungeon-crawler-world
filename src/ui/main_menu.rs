/// Main menu UI module for the Dungeon Crawler World application.
/// Handles rendering the main menu, settings, saves, and quit confirmation dialogs.
use egui::{Context, RichText};
use crate::logic::saves_logic::SaveMenuState;
use crate::ui::saves_ui::show_save_ui;
use crate::logic::settings_logic::{Settings, SettingsResult};
use crate::ui::settings_ui::settings_ui;

/// Represents the current state of the main menu.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuState {
    /// Main menu with buttons for Saves, Settings, and Quit
    Main,
    /// Currently showing settings
    Settings,
    /// Currently showing saves menu
    Saves,
}

/// Main menu UI state and logic
pub struct MainMenu {
    /// Current menu state (Main, Settings, or Saves)
    pub state: MenuState,
    /// State for the saves menu UI.
    pub save_menu_state: SaveMenuState,
    /// Quit confirmation dialog state.
    pub quit_confirm: bool,
}

impl MainMenu {
    /// Creates a new main menu with default state.
    pub fn new() -> Self {
        Self {
            state: MenuState::Main,
            save_menu_state: SaveMenuState::default(),
            quit_confirm: false,
        }
    }

    /// Renders the main menu UI based on current state.
    ///
    /// # Arguments
    /// * `ctx` - The egui context for UI rendering.
    /// * `settings` - Mutable reference to application settings.
    /// * `dev_mode_enabled` - Whether developer mode is enabled.
    ///
    /// # Returns
    /// `true` if the application should quit, `false` otherwise.
    pub fn show(&mut self, ctx: &Context, settings: &mut Settings, dev_mode_enabled: bool) -> bool {
        let mut should_quit = false;
        
        let escape_pressed: bool =
            ctx.input(|i: &egui::InputState| -> bool { i.key_pressed(egui::Key::Escape) });

        egui::CentralPanel::default()
            .frame(
                egui::Frame::central_panel(&ctx.style())
                    .inner_margin(egui::Margin::same(0))
                    .outer_margin(egui::Margin::same(0)),
            )
            .show(ctx, |ui: &mut egui::Ui| {
                let avail: egui::Vec2 = ui.available_size();
                ui.allocate_ui_with_layout(
                    avail,
                    egui::Layout::top_down(egui::Align::Center),
                    |ui: &mut egui::Ui| {
                        if self.state == MenuState::Settings {
                            ui.heading(RichText::new("Settings").size(28.0));
                            ui.add_space(8.0);
                            let mut back: bool = false;
                            egui::ScrollArea::vertical().auto_shrink([false; 2]).show(
                                ui,
                                |ui: &mut egui::Ui| {
                                    ui.set_max_width(700.0);
                                    let res: SettingsResult =
                                        settings_ui(ui, settings, dev_mode_enabled);
                                    if res.request_save {
                                        settings.save();
                                        self.state = MenuState::Main;
                                    }
                                    if res.request_back {
                                        back = true;
                                    }
                                },
                            );
                            if back || escape_pressed {
                                self.state = MenuState::Main;
                            }
                        } else if self.state == MenuState::Saves {
                            ui.heading(RichText::new("Saves Menu").size(28.0));
                            ui.add_space(8.0);
                            egui::ScrollArea::vertical().auto_shrink([false; 2]).show(
                                ui,
                                |ui: &mut egui::Ui| {
                                    ui.set_max_width(900.0);
                                    show_save_ui(ui, &mut self.save_menu_state, settings);
                                },
                            );
                            // Only close saves menu on explicit back, escape, or sub-menu exit
                            if self.save_menu_state.back_requested || escape_pressed {
                                self.save_menu_state.back_requested = false;
                                self.save_menu_state.in_new_save_menu = false;
                                self.save_menu_state.editing_save = None;
                                self.state = MenuState::Main;
                            }
                        } else {
                            ui.add_space(8.0);
                            ui.heading(RichText::new("Game Menu").size(30.0));
                            ui.add_space(24.0);
                            if ui.add_sized([220.0, 36.0], egui::Button::new("Saves")).clicked()
                            {
                                self.state = MenuState::Saves;
                            }
                            ui.add_space(8.0);
                            if ui.add_sized([220.0, 36.0], egui::Button::new("Settings"))
                                .clicked()
                            {
                                self.state = MenuState::Settings;
                            }
                            ui.add_space(8.0);
                            if ui.add_sized([220.0, 36.0], egui::Button::new("Quit")).clicked() {
                                self.quit_confirm = true;
                            }
                        }
                        // Quit confirmation dialog
                        if self.quit_confirm {
                            egui::Window::new("Quit Game?")
                                .collapsible(false)
                                .resizable(false)
                                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                                .show(ctx, |ui: &mut egui::Ui| {
                                    ui.label("Are you sure you want to quit?");
                                    ui.horizontal(|ui: &mut egui::Ui| {
                                        if ui.button("Yes").clicked() {
                                            should_quit = true;
                                            self.quit_confirm = false;
                                        }
                                        if ui.button("No").clicked() {
                                            self.quit_confirm = false;
                                        }
                                    });
                                });
                        }
                    },
                );
            });

        should_quit
    }
}

impl Default for MainMenu {
    fn default() -> Self {
        Self::new()
    }
}
