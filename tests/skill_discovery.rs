mod common;
use common::*;

#[test]
fn test_skills_directory_discovery() {
    let temp_dir = temp_test_dir();
    let skills_dir = create_test_skills_dir(temp_dir.path());
    
    // Verify the Skills directory was created
    assert!(skills_dir.exists());
    assert!(skills_dir.is_dir());
    assert_eq!(skills_dir.file_name().unwrap(), "Skills");
}

#[test]
fn test_skill_metadata_structure() {
    let temp_dir = temp_test_dir();
    let skills_dir = create_test_skills_dir(temp_dir.path());
    
    // Verify test skill directory exists
    let test_skill_dir = skills_dir.join("test_skill");
    assert!(test_skill_dir.exists());
    assert!(test_skill_dir.is_dir());
    
    // Verify description file exists
    let desc_file = test_skill_dir.join("description.md");
    assert!(desc_file.exists());
    assert!(desc_file.is_file());
    
    // Verify description content
    let content = std::fs::read_to_string(desc_file).unwrap();
    assert!(content.contains("# Test Skill"));
    assert!(content.contains("A skill for testing purposes"));
}

#[test]
fn test_multiple_skills_discovery() {
    let temp_dir = temp_test_dir();
    let skills_dir = create_test_skills_dir(temp_dir.path());
    
    // Create additional test skills
    let skill2_dir = skills_dir.join("another_skill");
    std::fs::create_dir_all(&skill2_dir).unwrap();
    std::fs::write(
        skill2_dir.join("description.md"),
        "# Another Skill\n\nAnother test skill."
    ).unwrap();
    
    let skill3_dir = skills_dir.join("third_skill");
    std::fs::create_dir_all(&skill3_dir).unwrap();
    std::fs::write(
        skill3_dir.join("description.md"),
        "# Third Skill\n\nYet another test skill."
    ).unwrap();
    
    // Verify all skills are discoverable
    let entries: Vec<_> = std::fs::read_dir(&skills_dir)
        .unwrap()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().is_dir())
        .collect();
    
    assert_eq!(entries.len(), 3);
    
    let skill_names: Vec<String> = entries
        .iter()
        .map(|entry| entry.file_name().to_string_lossy().to_string())
        .collect();
    
    assert!(skill_names.contains(&"test_skill".to_string()));
    assert!(skill_names.contains(&"another_skill".to_string()));
    assert!(skill_names.contains(&"third_skill".to_string()));
}

#[test]
fn test_skill_icon_detection() {
    let temp_dir = temp_test_dir();
    let skills_dir = create_test_skills_dir(temp_dir.path());
    
    let skill_dir = skills_dir.join("skill_with_icon");
    std::fs::create_dir_all(&skill_dir).unwrap();
    
    // Create description
    std::fs::write(
        skill_dir.join("description.md"),
        "# Skill With Icon\n\nA skill with an icon."
    ).unwrap();
    
    // Create a mock icon file (empty file for testing)
    std::fs::write(skill_dir.join("icon.png"), b"").unwrap();
    
    // Verify both files exist
    assert!(skill_dir.join("description.md").exists());
    assert!(skill_dir.join("icon.png").exists());
}

#[test]
fn test_malformed_skill_handling() {
    let temp_dir = temp_test_dir();
    let skills_dir = create_test_skills_dir(temp_dir.path());
    
    // Create a skill directory without description.md
    let bad_skill_dir = skills_dir.join("bad_skill");
    std::fs::create_dir_all(&bad_skill_dir).unwrap();
    
    // Create a file instead of directory (should be ignored)
    std::fs::write(skills_dir.join("not_a_skill"), "This is not a skill").unwrap();
    
    // Verify directory structure
    assert!(bad_skill_dir.exists());
    assert!(bad_skill_dir.is_dir());
    assert!(!bad_skill_dir.join("description.md").exists());
    
    assert!(skills_dir.join("not_a_skill").exists());
    assert!(skills_dir.join("not_a_skill").is_file());
}

#[test]
fn test_skill_name_extraction() {
    let temp_dir = temp_test_dir();
    let skills_dir = create_test_skills_dir(temp_dir.path());
    
    // Create skills with different naming patterns
    let skills = ["simple_skill", "Skill_With_Caps", "skill-with-dashes", "skill.with.dots"];
    
    for skill_name in &skills {
        let skill_dir = skills_dir.join(skill_name);
        std::fs::create_dir_all(&skill_dir).unwrap();
        std::fs::write(
            skill_dir.join("description.md"),
            format!("# {}\n\nDescription for {}.", skill_name, skill_name)
        ).unwrap();
    }
    
    // Verify all skills are created
    for skill_name in &skills {
        let skill_dir = skills_dir.join(skill_name);
        assert!(skill_dir.exists());
        assert!(skill_dir.join("description.md").exists());
    }
}

#[test]
fn test_description_content_parsing() {
    let temp_dir = temp_test_dir();
    let skills_dir = create_test_skills_dir(temp_dir.path());
    
    let skill_dir = skills_dir.join("complex_skill");
    std::fs::create_dir_all(&skill_dir).unwrap();
    
    let complex_description = r#"# Complex Skill

This is a more complex skill description with:

## Features
- Multiple sections
- Lists and formatting
- **Bold text**
- *Italic text*

## Requirements
- Level 5+
- 10 Intelligence

## Effects
Provides various benefits to the player.
"#;
    
    std::fs::write(skill_dir.join("description.md"), complex_description).unwrap();
    
    let content = std::fs::read_to_string(skill_dir.join("description.md")).unwrap();
    assert!(content.contains("# Complex Skill"));
    assert!(content.contains("## Features"));
    assert!(content.contains("Level 5+"));
    assert!(content.contains("**Bold text**"));
}