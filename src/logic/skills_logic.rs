use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use crate::player::Player;
use egui::TextureHandle;
use egui_commonmark::CommonMarkCache;

/// Public state to be held by the caller. Not integrated into the app here.
#[derive(Default)]
pub struct SkillsState {
    pub catalog: Vec<SkillMeta>,
    pub selected: Option<usize>,
    pub loaded: bool,
    /// When true, show all discovered skills as 'owned' for previewing
    pub show_all: bool,
    /// When true, show a dev-only Show All toggle
    pub dev_controls: bool,
    /// When true, hide non-owned skills from the grid
    pub only_owned: bool,
    /// Markdown render cache for images/assets
    pub md_cache: CommonMarkCache,
    /// Search query for filtering skills
    pub search: String,
    /// Sort mode: 0=Name A-Z, 1=Name Z-A, 2=Level High-Low, 3=Level Low-High
    pub sort_mode: u8,
    /// Current page number for pagination
    pub page: usize,
}

impl SkillsState {
    /// Enables preview mode, showing all discovered skills regardless of ownership.
    pub fn enable_preview(&mut self) {
        self.show_all = true;
    }

    /// Enables developer controls, exposing the Show All toggle button in the UI.
    pub fn enable_dev_controls(&mut self) {
        self.dev_controls = true;
    }
}

#[derive(Clone)]
pub struct SkillMeta {
    pub name: String,
    pub description: String,
    pub dir: PathBuf,
    pub icon: Option<TextureHandle>,
}

/// Attempts to find the root directory containing the Skills folder.
///
/// # Returns
/// * `Option<PathBuf>` - The path to the Skills root, or None if not found.
pub fn find_skills_root() -> Option<PathBuf> {
    // Try current working directory first
    if let Ok(cwd) = std::env::current_dir() {
        let p: PathBuf = cwd.join("Skills");
        if p.is_dir() {
            return Some(p);
        }
    }
    // Try relative to the executable (walk up a few parents)
    if let Ok(exe) = std::env::current_exe() {
        let mut dir_opt: Option<PathBuf> = exe.parent().map(|p| p.to_path_buf());
        for _ in 0..4 {
            if let Some(dir) = dir_opt.clone() {
                let candidate: PathBuf = dir.join("Skills");
                if candidate.is_dir() {
                    return Some(candidate);
                }
                dir_opt = dir.parent().map(|p| p.to_path_buf());
            }
        }
    }
    None
}

/// Reads the player's owned skills from disk.
///
/// # Returns
/// * `HashMap<String, i8>` - A map of skill names to their levels.
pub fn read_player_skills() -> HashMap<String, i8> {
    // Attempt to read current save context (if available)
    let current: Option<String> = crate::CURRENT_SAVE.lock().ok().and_then(|g| g.clone());
    let Some(save) = current else {
        return HashMap::new();
    };
    let path: PathBuf = Path::new("saves").join(save).join("player.json");
    let Ok(content) = fs::read_to_string(path) else {
        return HashMap::new();
    };
    let Ok(player) = serde_json::from_str::<Player>(&content) else {
        return HashMap::new();
    };
    player.skills
}
