use dungeon_crawler_world::logic::items_logic::read_potion_catalog_from;
use std::fs;

mod common;
use common::unique_temp_dir;

#[test]
fn reads_potion_metadata_and_description() {
    let root = unique_temp_dir("potion_catalog");
    let potion_dir = root.join("test_potion");
    fs::create_dir_all(&potion_dir).unwrap();
    fs::write(
        potion_dir.join("test.json"),
        r#"{
            "name": "Test Potion",
            "description": "description.md"
        }"#,
    )
    .unwrap();
    fs::write(potion_dir.join("description.md"), "A useful potion.").unwrap();

    let catalog = read_potion_catalog_from(&root);

    assert_eq!(catalog.len(), 1);
    assert_eq!(catalog[0].name, "Test Potion");
    assert_eq!(catalog[0].description, "A useful potion.");

    let _ = fs::remove_dir_all(root);
}
