use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use std::path::PathBuf;
use tempfile::TempDir;

/// Creates a deterministic RNG with a fixed seed for reproducible tests
pub fn test_rng() -> ChaCha8Rng {
    ChaCha8Rng::seed_from_u64(12345)
}

/// Creates a temporary directory for test isolation
pub fn temp_test_dir() -> TempDir {
    tempfile::tempdir().expect("Failed to create temp directory")
}

/// Creates a test Skills directory structure for skill discovery tests
pub fn create_test_skills_dir(base_dir: &std::path::Path) -> PathBuf {
    let skills_dir = base_dir.join("Skills");
    std::fs::create_dir_all(&skills_dir).expect("Failed to create Skills directory");
    
    // Create a sample skill directory
    let sample_skill = skills_dir.join("test_skill");
    std::fs::create_dir_all(&sample_skill).expect("Failed to create test skill directory");
    
    // Create description.md
    let desc_content = "# Test Skill\n\nA skill for testing purposes.\n";
    std::fs::write(
        sample_skill.join("description.md"),
        desc_content,
    ).expect("Failed to write description file");
    
    skills_dir
}

/// Creates a test saves directory structure
pub fn create_test_saves_dir(base_dir: &std::path::Path) -> PathBuf {
    let saves_dir = base_dir.join("saves");
    std::fs::create_dir_all(&saves_dir).expect("Failed to create saves directory");
    saves_dir
}

/// Helper assertions for testing player stats
pub mod assertions {
    use serde_json::Value;
    
    pub fn assert_stat_in_range(player_data: &Value, stat_name: &str, min: i64, max: i64) {
        let stat = player_data["stats"][stat_name]
            .as_i64()
            .unwrap_or_else(|| panic!("Stat '{}' not found or not an integer", stat_name));
        assert!(
            stat >= min && stat <= max,
            "Stat '{}' value {} is not in range [{}, {}]",
            stat_name,
            stat,
            min,
            max
        );
    }
    
    pub fn assert_skill_in_range(player_data: &Value, skill_name: &str, min: i64, max: i64) {
        let skill = player_data["skills"][skill_name]
            .as_i64()
            .unwrap_or_else(|| panic!("Skill '{}' not found or not an integer", skill_name));
        assert!(
            skill >= min && skill <= max,
            "Skill '{}' value {} is not in range [{}, {}]",
            skill_name,
            skill,
            min,
            max
        );
    }
}