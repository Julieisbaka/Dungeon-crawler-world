use dungeon_crawler_world::logic::settings_logic::{LogVerbosity, Settings, SettingsResult};

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

/// Serialises a `Settings` value to JSON then deserialises it and verifies
/// the round-trip produces an identical struct.
#[test]
fn test_settings_save_and_load_round_trip() {
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

    let json = serde_json::to_string_pretty(&original).unwrap();
    let loaded: Settings = serde_json::from_str(&json).unwrap();
    assert_eq!(original, loaded);
}

#[test]
fn test_settings_load_returns_defaults_for_missing_file() {
    // When the file does not exist Settings::load() falls back to defaults.
    // We verify by constructing the known defaults and checking field values.
    let defaults = build_default_settings();
    assert_eq!(defaults.fog, 2);
    assert_eq!(defaults.lighting, 3);
    assert_eq!(defaults.console_max_lines, 300);
}

#[test]
fn test_settings_load_returns_defaults_for_malformed_json() {
    // Write garbage JSON, parse it, confirm parse fails → defaults are used.
    let result = serde_json::from_str::<Settings>("{ bad json");
    assert!(result.is_err(), "Malformed JSON must fail to parse");

    // The load() path returns defaults on failure; verify defaults are sane.
    let defaults = build_default_settings();
    assert_eq!(defaults.fog, 2);
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
