use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ItemMeta {
    pub name: String,
    pub description: String,
    pub icon_path: Option<PathBuf>,
}

#[derive(Debug, Deserialize)]
struct ItemJson {
    name: String,
    description: Option<String>,
    icon: Option<String>,
}

pub fn read_potion_catalog() -> Vec<ItemMeta> {
    read_potion_catalog_from(Path::new("Items").join("Potions"))
}

pub fn read_potion_catalog_from(root: impl AsRef<Path>) -> Vec<ItemMeta> {
    let root = root.as_ref();
    let Ok(entries) = fs::read_dir(root) else {
        return Vec::new();
    };

    let mut items = Vec::new();
    for entry in entries.flatten() {
        let dir = entry.path();
        if !dir.is_dir() {
            continue;
        }
        let Ok(files) = fs::read_dir(&dir) else {
            continue;
        };
        for file in files.flatten() {
            let path = file.path();
            if path.extension().and_then(|extension| extension.to_str()) != Some("json") {
                continue;
            }
            if let Some(meta) = read_item_meta(&dir, &path) {
                items.push(meta);
            }
            break;
        }
    }

    items.sort_by(|left, right| left.name.cmp(&right.name));
    items
}

fn read_item_meta(dir: &Path, json_path: &Path) -> Option<ItemMeta> {
    let json = fs::read_to_string(json_path).ok()?;
    let data = serde_json::from_str::<ItemJson>(&json).ok()?;
    let description = data
        .description
        .as_deref()
        .and_then(|description_path| read_description(dir, description_path))
        .unwrap_or_default();
    let icon_path = data
        .icon
        .as_deref()
        .and_then(|icon| resolve_existing_path(dir, icon))
        .or_else(|| find_default_icon(dir));

    Some(ItemMeta {
        name: data.name,
        description,
        icon_path,
    })
}

fn read_description(dir: &Path, description_path: &str) -> Option<String> {
    let path = resolve_existing_path(dir, description_path)
        .unwrap_or_else(|| dir.join(Path::new(description_path).file_name().unwrap_or_default()));
    fs::read_to_string(path).ok()
}

fn resolve_existing_path(dir: &Path, path: &str) -> Option<PathBuf> {
    let direct = PathBuf::from(path);
    if direct.exists() {
        return Some(direct);
    }
    let file_name = Path::new(path).file_name()?;
    let local = dir.join(file_name);
    local.exists().then_some(local)
}

fn find_default_icon(dir: &Path) -> Option<PathBuf> {
    ["icon.png", "icon.jpg", "icon.jpeg", "icon.webp"]
        .into_iter()
        .map(|name| dir.join(name))
        .find(|path| path.exists())
}
