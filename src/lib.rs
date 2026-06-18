pub mod logic;
pub mod ui;
pub mod new_save;
pub mod console;
pub mod game;
pub mod grid;
pub mod save_game;
pub mod terrain3d;

pub use new_save::{generate_floor_one_time, generate_stats};

pub mod player;

// Global save tracking used by various UI/logic modules
use once_cell::sync::Lazy;
use std::sync::Mutex;

pub static CURRENT_SAVE: Lazy<Mutex<Option<String>>> = Lazy::new(|| Mutex::new(None));
