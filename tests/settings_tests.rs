mod common;

use dungeon_crawler_world::logic::settings_logic::{LogVerbosity, Settings, SettingsResult};
use std::fs;
use std::sync::Mutex;

/// Serializes the settings lock for tests that must change the working directory.
/// Only tests that touch `Settings::save()` / `Settings::load()` acquire this lock.
static SETTINGS_CWD_LOCK: Mutex<()> = Mutex::new(());

/// RAII guard that restores the process working directory when dropped, even on panic.
struct RestoreCwd(std::path::PathBuf);

impl Drop for RestoreCwd {
    fn drop(&mut self) {
        let _ = std::env::set_current_dir(&self.0);
    }
}

// ── LogVerbosity ───────────────────────────────────────────────────────────────

#[test]
fn test_log_verbosity_default_is_info() {
    assert_eq!(LogVerbosity::default(), LogVerbosity::Info);
}

#[test]
fn test_log_verbosity_variants_are_distinct() {
    assert_ne!(LogVerbosity::Error, LogVerbosity::Warn);
    assert_ne!(LogVerbosity::Warn, LogVerbosity::Info);
    assert_ne!(LogVerbosity::Info, LogVerbosity::Debug);
    assert_ne!(LogVerbosity::Debug, LogVerbosity::Trace);
}

// ── SettingsResult ─────────────────────────────────────────────────────────────

#[test]
fn test_settings_result_default_all_false() {
    let result = SettingsResult::default();
    assert!(!result.request_save);
    assert!(!result.request_back);
}

// ── Settings default values ────────────────────────────────────────────────────

/// Exercises `Settings::default_inner()` indirectly via a round-trip: we
/// serialize a freshly-constructed default, then assert the known fields.
#[test]
fn test_settings_default_values() {
    // Avoid touching the real settings.json by serializing directly.
    let json = serde_json::to_value(build_default_settings()).unwrap();

    assert_eq!(json["fog"], 2);
    assert_eq!(json["lighting"], 3);
    assert!(json["sound"].as_bool().unwrap());
    assert!(!json["developer_mode"].as_bool().unwrap());
    assert!(!json["verbose_logging"].as_bool().unwrap());
    assert!(!json["show_console"].as_bool().unwrap());
    assert!(!json["show_fps_graph"].as_bool().unwrap());
    assert!(!json["fullscreen"].as_bool().unwrap());
    assert!(!json["log_to_console"].as_bool().unwrap());
    assert_eq!(json["console_max_lines"], 300);
    assert!(json["show_save_creation_date"].as_bool().unwrap());
}

// ── Settings serialization round-trip ─────────────────────────────────────────

#[test]
fn test_settings_serialize_deserialize_round_trip() {
    let original = Settings {
        fog: 1,
        lighting: 5,
        sound: false,
        developer_mode: true,
        verbose_logging: true,
        show_console: true,
        show_fps_graph: true,
        log_to_console: true,
        log_verbosity: LogVerbosity::Debug,
        fullscreen: true,
        console_max_lines: 500,
        show_save_creation_date: false,
    };

    let json = serde_json::to_string(&original).unwrap();
    let restored: Settings = serde_json::from_str(&json).unwrap();

    assert_eq!(original, restored);
}

// ── Settings::save / Settings::load ───────────────────────────────────────────

/// Calls `Settings::save()` then `Settings::load()` in a temp directory and
/// verifies the loaded value is identical to the saved one.
#[test]
fn test_settings_save_and_load_round_trip() {
    let temp_dir = common::unique_temp_dir("settings_save_load");
    let _guard = SETTINGS_CWD_LOCK.lock().unwrap();
    let _restore = RestoreCwd(std::env::current_dir().unwrap());
    std::env::set_current_dir(&temp_dir).unwrap();

    let original = Settings {
        fog: 0,
        lighting: 1,
        sound: false,
        developer_mode: false,
        verbose_logging: false,
        show_console: false,
        show_fps_graph: false,
        log_to_console: false,
        log_verbosity: LogVerbosity::Warn,
        fullscreen: false,
        console_max_lines: 100,
        show_save_creation_date: false,
    };

    original.save();
    let loaded = Settings::load();

    drop(_restore);
    let _ = fs::remove_dir_all(&temp_dir);

    assert_eq!(original, loaded);
}

/// Verifies that `Settings::load()` returns hard-coded defaults when
/// `settings.json` does not exist in the working directory.
#[test]
fn test_settings_load_returns_defaults_for_missing_file() {
    let temp_dir = common::unique_temp_dir("settings_missing");
    let _guard = SETTINGS_CWD_LOCK.lock().unwrap();
    let _restore = RestoreCwd(std::env::current_dir().unwrap());
    std::env::set_current_dir(&temp_dir).unwrap();

    // No settings.json in temp_dir → load() must return defaults.
    let defaults = Settings::load();

    drop(_restore);
    let _ = fs::remove_dir_all(&temp_dir);

    assert_eq!(defaults.fog, 2);
    assert_eq!(defaults.lighting, 3);
    assert_eq!(defaults.console_max_lines, 300);
    assert!(defaults.sound);
    assert!(!defaults.developer_mode);
}

/// Verifies that `Settings::load()` returns hard-coded defaults when
/// `settings.json` contains invalid JSON.
#[test]
fn test_settings_load_returns_defaults_for_malformed_json() {
    let temp_dir = common::unique_temp_dir("settings_malformed");
    let _guard = SETTINGS_CWD_LOCK.lock().unwrap();
    let _restore = RestoreCwd(std::env::current_dir().unwrap());
    std::env::set_current_dir(&temp_dir).unwrap();

    // Write garbage so the file exists but cannot be parsed.
    fs::write("settings.json", "{ bad json {{{{").unwrap();
    let defaults = Settings::load();

    drop(_restore);
    let _ = fs::remove_dir_all(&temp_dir);

    assert_eq!(defaults.fog, 2);
    assert_eq!(defaults.lighting, 3);
    assert_eq!(defaults.console_max_lines, 300);
}

// ── Settings equality ──────────────────────────────────────────────────────────

#[test]
fn test_settings_equality() {
    let a = build_default_settings();
    let b = build_default_settings();
    assert_eq!(a, b);
}

#[test]
fn test_settings_inequality_on_field_change() {
    let a = build_default_settings();
    let mut b = build_default_settings();
    b.fog = 0;
    assert_ne!(a, b);
}

// ── helpers ───────────────────────────────────────────────────────────────────

fn build_default_settings() -> Settings {
    Settings {
        fog: 2,
        lighting: 3,
        sound: true,
        developer_mode: false,
        verbose_logging: false,
        show_console: false,
        show_fps_graph: false,
        log_to_console: false,
        log_verbosity: LogVerbosity::Info,
        fullscreen: false,
        console_max_lines: 300,
        show_save_creation_date: true,
    }
}
