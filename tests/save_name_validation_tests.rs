use dungeon_crawler_world::new_save::{has_invalid_save_characters, is_safe_folder_name};

#[test]
fn rejects_relative_paths() {
    assert!(!is_safe_folder_name("."));
    assert!(!is_safe_folder_name(".."));
    assert!(!is_safe_folder_name("../foo"));
    assert!(!is_safe_folder_name("foo/../bar"));
    assert!(!is_safe_folder_name("foo/./bar"));
}

#[test]
fn accepts_simple_names() {
    assert!(is_safe_folder_name("Test_Save"));
    assert!(is_safe_folder_name("save1"));
}

#[test]
fn rejects_disallowed_characters() {
    assert!(has_invalid_save_characters("bad/name"));
    assert!(has_invalid_save_characters("bad:name"));
    assert!(has_invalid_save_characters("bad*name"));
    assert!(!has_invalid_save_characters("Good_Name"));
}
