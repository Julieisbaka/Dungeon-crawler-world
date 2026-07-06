use crate::input::{GameCommand, KeyBindings};
use egui::{Context, RichText, Ui};

#[derive(Default)]
pub struct KeybindsUiState {
    waiting_for_key: Option<GameCommand>,
}

impl KeybindsUiState {
    pub fn clear_capture(&mut self) {
        self.waiting_for_key = None;
    }
}

pub fn capture_keybind_input(
    ctx: &Context,
    keybindings: &mut KeyBindings,
    state: &mut KeybindsUiState,
) {
    let Some(command) = state.waiting_for_key else {
        return;
    };

    let pressed_key = ctx.input(|input| {
        input.events.iter().find_map(|event| match event {
            egui::Event::Key {
                key,
                pressed: true,
                repeat: false,
                ..
            } => Some(*key),
            _ => None,
        })
    });

    if let Some(key) = pressed_key {
        keybindings.set_key(command, key);
        state.waiting_for_key = None;
    }
}

pub fn keybinds_ui(ui: &mut Ui, keybindings: &mut KeyBindings, state: &mut KeybindsUiState) {
    ui.label("Select a command, then press the new key.");
    ui.add_space(8.0);
    keybind_row(ui, keybindings, state, GameCommand::Skills, "Skills");
    keybind_row(ui, keybindings, state, GameCommand::Inventory, "Inventory");

    if let Some(command) = state.waiting_for_key {
        ui.add_space(8.0);
        ui.label(RichText::new(format!("Waiting for {:?} key...", command)).italics());
    }
}

fn keybind_row(
    ui: &mut Ui,
    keybindings: &mut KeyBindings,
    state: &mut KeybindsUiState,
    command: GameCommand,
    label: &str,
) {
    ui.horizontal(|ui| {
        ui.label(label);
        ui.monospace(format!("{:?}", keybindings.key_for(command)));
        if ui.button("Change").clicked() {
            state.waiting_for_key = Some(command);
        }
    });
}
