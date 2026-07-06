#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameCommand {
    Skills,
    Inventory,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyBindings {
    pub skills: egui::Key,
    pub inventory: egui::Key,
}

impl Default for KeyBindings {
    fn default() -> Self {
        Self {
            skills: egui::Key::H,
            inventory: egui::Key::E,
        }
    }
}

impl KeyBindings {
    pub fn key_for(&self, command: GameCommand) -> egui::Key {
        match command {
            GameCommand::Skills => self.skills,
            GameCommand::Inventory => self.inventory,
        }
    }

    pub fn set_key(&mut self, command: GameCommand, key: egui::Key) {
        match command {
            GameCommand::Skills => self.skills = key,
            GameCommand::Inventory => self.inventory = key,
        }
    }
}
