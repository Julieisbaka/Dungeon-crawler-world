/// Main menu UI module for the Dungeon Crawler World application.
/// Handles rendering the main menu, settings, saves, and quit confirmation dialogs.

use egui::{Context, RichText};
use crate::logic::saves_logic::SaveMenuState;
use crate::player::PlayerPosition;
use crate::save_game::save_player_state;
use crate::terrain3d::TerrainPoint;
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
    /// Active loaded 3D terrain gameplay view
    Game,
}

/// Main menu UI state and logic
pub struct MainMenu {
    /// Current menu state (Main, Settings, or Saves)
    pub state: MenuState,
    /// State for the saves menu UI.
    pub save_menu_state: SaveMenuState,
    /// Quit confirmation dialog state.
    pub quit_confirm: bool,
    last_player_state_save: Option<std::time::Instant>,
    player_state_dirty: bool,
}

impl MainMenu {
    /// Creates a new main menu with default state.
    pub fn new() -> Self {
        Self {
            state: MenuState::Main,
            save_menu_state: SaveMenuState::default(),
            quit_confirm: false,
            last_player_state_save: None,
            player_state_dirty: false,
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
    pub fn show(
        &mut self,
        ctx: &Context,
        settings: &mut Settings,
        dev_mode_enabled: bool,
        raw_mouse_delta: egui::Vec2,
    ) -> bool {
        let mut should_quit = false;
        
        let escape_pressed: bool =
            ctx.input(|i: &egui::InputState| -> bool { i.key_pressed(egui::Key::Escape) });

        if self.state == MenuState::Game {
            self.show_game_overlay(ctx, escape_pressed, raw_mouse_delta);
            return false;
        }

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
                            if self.save_menu_state.enter_loaded_save_requested {
                                self.save_menu_state.enter_loaded_save_requested = false;
                                self.state = MenuState::Game;
                            }
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

    fn show_game_overlay(
        &mut self,
        ctx: &Context,
        escape_pressed: bool,
        raw_mouse_delta: egui::Vec2,
    ) {
        let save_status = {
            let Some(save) = self.save_menu_state.loaded_save.as_mut() else {
                self.state = MenuState::Main;
                return;
            };

            if apply_game_controls(ctx, raw_mouse_delta, save) {
                self.player_state_dirty = true;
            }

            format!(
                "{} | Floor {} | {:.1}, {:.1}, {:.1}",
                save.folder_name,
                save.player.current_floor,
                save.player.position.x,
                save.player.position.y,
                save.player.position.z
            )
        };

        self.flush_player_state_if_due(false);

        let mut exit_to_menu = escape_pressed;
        egui::Area::new(egui::Id::new("game_overlay"))
            .anchor(egui::Align2::LEFT_TOP, egui::vec2(12.0, 12.0))
            .show(ctx, |ui| {
                egui::Frame::NONE
                    .fill(egui::Color32::from_black_alpha(150))
                    .inner_margin(egui::Margin::same(8))
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            if ui.button("Menu").clicked() {
                                exit_to_menu = true;
                            }
                            ui.label(&save_status);
                        });
                    });
            });

        if exit_to_menu {
            self.flush_player_state_if_due(true);
            self.state = MenuState::Main;
        }
    }

    fn flush_player_state_if_due(&mut self, force: bool) {
        if !self.player_state_dirty {
            return;
        }

        let now = std::time::Instant::now();
        if !force
            && self
                .last_player_state_save
                .is_some_and(|last_save| now.duration_since(last_save) < std::time::Duration::from_millis(300))
        {
            return;
        }

        let Some(save) = self.save_menu_state.loaded_save.as_ref() else {
            return;
        };
        if let Err(error) = save_player_state(
            std::path::Path::new("saves"),
            &save.folder_name,
            &save.player,
        ) {
            self.save_menu_state.load_error = Some(error);
            return;
        }

        self.last_player_state_save = Some(now);
        self.player_state_dirty = false;
    }
}

fn apply_game_controls(
    ctx: &Context,
    raw_mouse_delta: egui::Vec2,
    save: &mut crate::save_game::SaveGame,
) -> bool {
    let mut strafe: f32 = 0.0;
    let mut forward_amount: f32 = 0.0;
    ctx.input(|input| {
        if input.key_down(egui::Key::W) || input.key_down(egui::Key::ArrowUp) {
            forward_amount += 1.0;
        }
        if input.key_down(egui::Key::S) || input.key_down(egui::Key::ArrowDown) {
            forward_amount -= 1.0;
        }
        if input.key_down(egui::Key::A) || input.key_down(egui::Key::ArrowLeft) {
            strafe -= 1.0;
        }
        if input.key_down(egui::Key::D) || input.key_down(egui::Key::ArrowRight) {
            strafe += 1.0;
        }
    });

    let mut changed = false;
    if raw_mouse_delta != egui::Vec2::ZERO {
        let sensitivity = 0.0012;
        save.player.look.yaw += raw_mouse_delta.x * sensitivity;
        save.player.look.pitch = (save.player.look.pitch - raw_mouse_delta.y * sensitivity)
            .clamp(-1.2, 1.2);
        changed = true;
    }

    if strafe != 0.0 || forward_amount != 0.0 {
        let yaw = save.player.look.yaw;
        let forward_x = yaw.sin();
        let forward_z = -yaw.cos();
        let right_x = yaw.cos();
        let right_z = yaw.sin();
        let move_x = right_x * strafe + forward_x * forward_amount;
        let move_z = right_z * strafe + forward_z * forward_amount;
        let length = f32::sqrt(move_x * move_x + move_z * move_z);
        let speed = save.world.temporary_character.speed;
        let requested = PlayerPosition::new(
            save.player.position.x + move_x / length * speed,
            save.player.position.y,
            save.player.position.z + move_z / length * speed,
        );
        save.player.position = save
            .collision_map
            .constrain_movement(save.player.position, requested);
        save.world.temporary_character.position = TerrainPoint::new(
            save.player.position.x,
            save.player.position.y,
            save.player.position.z,
        );
        changed = true;
    }

    changed
}

impl Default for MainMenu {
    fn default() -> Self {
        Self::new()
    }
}
