use crate::input::KeyBindings;
use crate::ui::inventory_ui::{inventory_ui, InventoryUiState};
use crate::ui::keybinds_ui::{capture_keybind_input, keybinds_ui, KeybindsUiState};
use crate::ui::pause_menu::{pause_menu_ui, PauseMenuAction};
use crate::ui::skills_ui::skills_ui;
use crate::ui::stats_ui::stats_ui;
use crate::world::WorldSession;
use egui::{Align2, Context, FontId, Vec2};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameViewAction {
    None,
    ExitToMenu,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GamePanel {
    None,
    Skills,
    Inventory,
    Stats,
    Keybinds,
}

pub struct GameViewState {
    pub skills: crate::logic::skills_logic::SkillsState,
    pub inventory: InventoryUiState,
    pub active_panel: GamePanel,
    pub keybindings: KeyBindings,
    pub keybinds: KeybindsUiState,
}

impl Default for GameViewState {
    fn default() -> Self {
        Self {
            skills: crate::logic::skills_logic::SkillsState::default(),
            inventory: InventoryUiState::default(),
            active_panel: GamePanel::None,
            keybindings: KeyBindings::default(),
            keybinds: KeybindsUiState::default(),
        }
    }
}

impl GameViewState {
    pub fn open_panel(&mut self, panel: GamePanel) {
        self.active_panel = panel;
    }

    pub fn close_panel(&mut self) {
        self.active_panel = GamePanel::None;
        self.keybinds.clear_capture();
    }
}

pub fn show_game_view(
    ctx: &Context,
    world: &mut WorldSession,
    ui_state: &mut GameViewState,
) -> GameViewAction {
    capture_keybind_input(ctx, &mut ui_state.keybindings, &mut ui_state.keybinds);

    show_world_hud(ctx, world);

    let mut action = GameViewAction::None;
    if world.paused {
        match pause_menu_ui(ctx) {
            PauseMenuAction::None => {}
            PauseMenuAction::Resume => world.resume(),
            PauseMenuAction::OpenKeybinds => ui_state.open_panel(GamePanel::Keybinds),
            PauseMenuAction::OpenSkills => ui_state.open_panel(GamePanel::Skills),
            PauseMenuAction::OpenInventory => ui_state.open_panel(GamePanel::Inventory),
            PauseMenuAction::OpenStats => ui_state.open_panel(GamePanel::Stats),
            PauseMenuAction::ExitToMenu => {
                ui_state.close_panel();
                action = GameViewAction::ExitToMenu;
            }
        }
    }

    show_active_panel(ctx, world, ui_state);
    action
}

fn show_world_hud(ctx: &Context, world: &WorldSession) {
    let current = world.current_cell();
    egui::Area::new(egui::Id::new("world_hud"))
        .anchor(Align2::LEFT_TOP, Vec2::new(16.0, 14.0))
        .interactable(false)
        .show(ctx, |ui| {
            ui.label(
                egui::RichText::new(format!(
                    "{} | Cell {}, {}, {} | Chunks {}",
                    world.save_name,
                    current.x,
                    current.y,
                    current.z,
                    world.terrain.generated_chunk_count()
                ))
                .font(FontId::monospace(14.0)),
            );
        });

    egui::Area::new(egui::Id::new("world_controls_hint"))
        .anchor(Align2::LEFT_BOTTOM, Vec2::new(16.0, -16.0))
        .interactable(false)
        .show(ctx, |ui| {
            ui.label(
                egui::RichText::new(
                    "Mouse turns. WASD/arrow keys move. Space jumps. Esc pauses. H skills. E inventory. C stats.",
                )
                .font(FontId::monospace(12.0)),
            );
        });
}

fn show_active_panel(ctx: &Context, world: &mut WorldSession, ui_state: &mut GameViewState) {
    match ui_state.active_panel {
        GamePanel::None => {}
        GamePanel::Skills => {
            let mut open = true;
            egui::Window::new("Skills")
                .open(&mut open)
                .resizable(true)
                .default_size([760.0, 520.0])
                .show(ctx, |ui| skills_ui(ui, &mut ui_state.skills));
            if !open {
                ui_state.close_panel();
            }
        }
        GamePanel::Inventory => {
            let mut open = true;
            egui::Window::new("Inventory")
                .open(&mut open)
                .resizable(true)
                .default_size([420.0, 420.0])
                .show(ctx, |ui| {
                    inventory_ui(ui, &world.player, &mut ui_state.inventory)
                });
            if !open {
                ui_state.close_panel();
            }
        }
        GamePanel::Stats => {
            let mut open = true;
            egui::Window::new("Stats")
                .open(&mut open)
                .resizable(false)
                .default_size([320.0, 300.0])
                .show(ctx, |ui| stats_ui(ui, &world.player));
            if !open {
                ui_state.close_panel();
            }
        }
        GamePanel::Keybinds => {
            let mut open = true;
            egui::Window::new("Keybinds")
                .open(&mut open)
                .resizable(false)
                .default_size([360.0, 180.0])
                .show(ctx, |ui| {
                    keybinds_ui(ui, &mut ui_state.keybindings, &mut ui_state.keybinds)
                });
            if !open {
                ui_state.close_panel();
            }
        }
    }
}
