use egui::{Align2, Context};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PauseMenuAction {
    None,
    Resume,
    OpenKeybinds,
    OpenSkills,
    OpenInventory,
    ExitToMenu,
}

pub fn pause_menu_ui(ctx: &Context) -> PauseMenuAction {
    let mut action = PauseMenuAction::None;

    egui::Window::new("Paused")
        .collapsible(false)
        .resizable(false)
        .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
        .show(ctx, |ui| {
            ui.set_min_width(260.0);
            if ui.button("Resume").clicked() {
                action = PauseMenuAction::Resume;
            }
            if ui.button("Keybinds").clicked() {
                action = PauseMenuAction::OpenKeybinds;
            }
            if ui.button("Skills").clicked() {
                action = PauseMenuAction::OpenSkills;
            }
            if ui.button("Inventory").clicked() {
                action = PauseMenuAction::OpenInventory;
            }
            ui.separator();
            if ui.button("Exit to Menu").clicked() {
                action = PauseMenuAction::ExitToMenu;
            }
        });

    action
}
