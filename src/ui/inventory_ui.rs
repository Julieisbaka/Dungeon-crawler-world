use crate::logic::items_logic::{read_potion_catalog, ItemMeta};
use crate::player::Player;
use egui::{Color32, ColorImage, FontId, Pos2, Rect, Stroke, TextureHandle, Ui, Vec2};
use image::{GenericImageView, ImageReader};
use std::collections::HashMap;
use std::path::Path;

const HOT_SLOT_COUNT: usize = 9;
const HOT_SLOT_STACK_LIMIT: i32 = 999;

#[derive(Default)]
pub struct InventoryUiState {
    potions: Vec<ItemMeta>,
    textures: HashMap<String, TextureHandle>,
    loaded: bool,
}

pub fn inventory_ui(ui: &mut Ui, player: &Player, state: &mut InventoryUiState) {
    if !state.loaded {
        state.potions = read_potion_catalog();
        state.loaded = true;
    }

    ui.heading("Inventory");
    ui.label(format!("Coins: {}", player.coins));
    ui.separator();

    ui.heading("Hot Slots");
    ui.horizontal_wrapped(|ui| {
        for slot in hot_slots(player, &state.potions) {
            draw_hot_slot(ui, state, slot.as_ref());
        }
    });

    ui.separator();
    ui.heading("Backpack");
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
}

struct HotSlot {
    item: ItemMeta,
    count: i32,
}

fn hot_slots(player: &Player, potions: &[ItemMeta]) -> Vec<Option<HotSlot>> {
    let mut slots = Vec::with_capacity(HOT_SLOT_COUNT);

    for potion in potions {
        let mut remaining = player.inventory.get(&potion.name).copied().unwrap_or(0);
        while remaining > 0 && slots.len() < HOT_SLOT_COUNT {
            let count = remaining.min(HOT_SLOT_STACK_LIMIT);
            slots.push(Some(HotSlot {
                item: potion.clone(),
                count,
            }));
            remaining -= count;
        }
        if slots.len() == HOT_SLOT_COUNT {
            break;
        }
    }

    while slots.len() < HOT_SLOT_COUNT {
        slots.push(None);
    }

    slots
}

fn draw_hot_slot(ui: &mut Ui, state: &mut InventoryUiState, slot: Option<&HotSlot>) {
    let size = Vec2::splat(54.0);
    let (rect, response) = ui.allocate_exact_size(size, egui::Sense::hover());
    let painter = ui.painter_at(rect);
    painter.rect_filled(rect, 4.0, Color32::from_rgb(26, 30, 32));
    painter.rect_stroke(
        rect,
        4.0,
        Stroke::new(1.0, Color32::from_rgb(86, 96, 94)),
        egui::StrokeKind::Inside,
    );

    if let Some(slot) = slot {
        draw_icon(ui, state, rect.shrink(6.0), &slot.item);
        let count_text = slot.count.to_string();
        let count_pos = rect.right_bottom() - Vec2::new(5.0, 4.0);
        painter.text(
            count_pos,
            egui::Align2::RIGHT_BOTTOM,
            count_text,
            FontId::monospace(12.0),
            Color32::WHITE,
        );
        response.on_hover_ui(|ui| {
            ui.strong(&slot.item.name);
            if !slot.item.description.trim().is_empty() {
                ui.separator();
                ui.label(slot.item.description.trim());
            }
        });
    }
}

fn draw_icon(ui: &mut Ui, state: &mut InventoryUiState, rect: Rect, item: &ItemMeta) {
    if let Some(texture) = item
        .icon_path
        .as_deref()
        .and_then(|path| load_texture(ui, &mut state.textures, &item.name, path))
    {
        ui.painter().image(
            texture.id(),
            rect,
            Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0)),
            Color32::WHITE,
        );
        return;
    }

    ui.painter()
        .rect_filled(rect, 4.0, Color32::from_rgb(72, 45, 86));
    ui.painter().circle_filled(
        rect.center() + Vec2::new(0.0, 4.0),
        rect.width() * 0.2,
        Color32::from_rgb(166, 82, 195),
    );
    ui.painter().rect_filled(
        Rect::from_center_size(rect.center() - Vec2::new(0.0, 10.0), Vec2::new(14.0, 7.0)),
        2.0,
        Color32::from_rgb(188, 202, 206),
    );
}

fn load_texture<'a>(
    ui: &Ui,
    textures: &'a mut HashMap<String, TextureHandle>,
    key: &str,
    path: &Path,
) -> Option<&'a TextureHandle> {
    if !textures.contains_key(key) {
        let reader = ImageReader::open(path).ok()?;
        let image = reader.decode().ok()?;
        let size = image.dimensions();
        let rgba = image.to_rgba8();
        let pixels = rgba.as_flat_samples();
        let color_image = ColorImage::from_rgba_unmultiplied(
            [size.0 as usize, size.1 as usize],
            pixels.as_slice(),
        );
        let texture = ui.ctx().load_texture(
            format!("inventory_item_{key}"),
            color_image,
            egui::TextureOptions::default(),
        );
        textures.insert(key.to_string(), texture);
    }

    textures.get(key)
}
