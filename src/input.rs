#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameCommand {
    Skills,
    Inventory,
    Stats,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyBindings {
    pub skills: egui::Key,
    pub inventory: egui::Key,
    pub stats: egui::Key,
}

impl Default for KeyBindings {
    fn default() -> Self {
        Self {
            skills: egui::Key::H,
            inventory: egui::Key::E,
            stats: egui::Key::C,
        }
    }
}

impl KeyBindings {
    pub fn key_for(&self, command: GameCommand) -> egui::Key {
        match command {
            GameCommand::Skills => self.skills,
            GameCommand::Inventory => self.inventory,
            GameCommand::Stats => self.stats,
        }
    }

    pub fn set_key(&mut self, command: GameCommand, key: egui::Key) {
        match command {
            GameCommand::Skills => self.skills = key,
            GameCommand::Inventory => self.inventory = key,
            GameCommand::Stats => self.stats = key,
        }
    }
}
