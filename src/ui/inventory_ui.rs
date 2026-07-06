use crate::player::Player;
use egui::Ui;

pub fn inventory_ui(ui: &mut Ui, player: &Player) {
    ui.heading("Inventory");
    ui.label(format!("Coins: {}", player.coins));
    ui.separator();

    if player.inventory.is_empty() {
        ui.label("Your pack is empty.");
    } else {
        egui::Grid::new("inventory_items_grid")
            .striped(true)
            .spacing([16.0, 6.0])
            .show(ui, |ui| {
                ui.strong("Item");
                ui.strong("Count");
                ui.end_row();

                let mut items: Vec<_> = player.inventory.iter().collect();
                items.sort_by(|left, right| left.0.cmp(right.0));
                for (name, count) in items {
                    ui.label(name);
                    ui.label(count.to_string());
                    ui.end_row();
                }
            });
    }

    ui.separator();
    ui.heading("Stats");
    egui::Grid::new("inventory_stats_grid")
        .striped(true)
        .spacing([16.0, 6.0])
        .show(ui, |ui| {
            ui.label("Strength");
            ui.label(player.stats.strength.to_string());
            ui.end_row();
            ui.label("Intelligence");
            ui.label(player.stats.intelligence.to_string());
            ui.end_row();
            ui.label("Dexterity");
            ui.label(player.stats.dexterity.to_string());
            ui.end_row();
            ui.label("Charisma");
            ui.label(player.stats.charisma.to_string());
            ui.end_row();
            ui.label("Constitution");
            ui.label(player.stats.constitution.to_string());
            ui.end_row();
        });
}
