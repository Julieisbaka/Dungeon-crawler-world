use crate::new_save::NewSaveState;
use egui::TextureHandle;
use std::collections::HashMap;

/// Cached data for a single save entry to avoid repeated disk reads.
#[derive(Clone)]
pub struct SaveEntryCache {
    pub folder_name: String,
    pub save_name: String,
    pub difficulty_text: String,
    pub created_at_text: String,
    pub icon: Option<TextureHandle>,
}

pub struct SaveMenuState {
    #[allow(dead_code)]
    pub show_menu: bool,
    pub new_save_state: NewSaveState,
    pub in_new_save_menu: bool,
    pub editing_save: Option<String>,
    pub edit_save_name: String,
    pub confirm_delete: bool,
    pub delete_target: Option<String>,
    // Set to true when the top-level Back is clicked; caller can observe and react
    pub back_requested: bool,
    /// Cached save entries to avoid repeated disk reads and image decoding.
    /// Key is the folder name.
    pub save_cache: HashMap<String, SaveEntryCache>,
    /// Whether the save cache has been loaded.
    pub cache_loaded: bool,
}

impl Default for SaveMenuState {
    /// Returns a new `SaveMenuState` with default values for all fields.
    fn default() -> Self {
        Self {
            show_menu: false,
            new_save_state: NewSaveState::default(),
            in_new_save_menu: false,
            editing_save: None,
            edit_save_name: String::new(),
            confirm_delete: false,
            delete_target: None,
            back_requested: false,
            save_cache: HashMap::new(),
            cache_loaded: false,
        }
    }
}

impl SaveMenuState {
    /// Invalidates the save cache, forcing a reload on next render.
    /// Call this after creating, renaming, or deleting saves.
    pub fn invalidate_cache(&mut self) {
        self.save_cache.clear();
        self.cache_loaded = false;
    }
}
