use crate::player::Player;
use egui::Ui;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct StatUpgradeVisibility {
    pub strength: bool,
    pub intelligence: bool,
    pub dexterity: bool,
    pub charisma: bool,
    pub constitution: bool,
}

pub fn stats_ui(ui: &mut Ui, player: &Player) {
    stats_ui_with_upgrades(ui, player, StatUpgradeVisibility::default());
}

pub fn stats_ui_with_upgrades(ui: &mut Ui, player: &Player, upgrades: StatUpgradeVisibility) {
    ui.heading("Stats");
    ui.separator();

    ui.label(format!("Level {}", player.level.max(1)));
    ui.add(
        egui::ProgressBar::new(0.0)
            .show_percentage()
            .text("0 / 100 XP"),
    );
    ui.add_space(8.0);

    egui::Grid::new("player_stats_grid")
        .striped(true)
        .spacing([18.0, 8.0])
        .show(ui, |ui| {
            stat_row(ui, "Floor", player.current_floor.to_string(), false);
            stat_row(
                ui,
                "Strength",
                player.stats.strength.to_string(),
                upgrades.strength,
            );
            stat_row(
                ui,
                "Intelligence",
                player.stats.intelligence.to_string(),
                upgrades.intelligence,
            );
            stat_row(
                ui,
                "Dexterity",
                player.stats.dexterity.to_string(),
                upgrades.dexterity,
            );
            stat_row(
                ui,
                "Charisma",
                player.stats.charisma.to_string(),
                upgrades.charisma,
            );
            stat_row(
                ui,
                "Constitution",
                player.stats.constitution.to_string(),
                upgrades.constitution,
            );
        });
}

fn stat_row(ui: &mut Ui, label: &str, value: String, show_plus: bool) {
    ui.label(label);
    ui.label(value);
    if show_plus {
        let _ = ui.button("+");
    } else {
        ui.label("");
    }
    ui.end_row();
}
