// Speed = log(str + 1) + ln(dex + 1)

/// Calculate player speed based on strength and dexterity
pub fn calculate_speed(strength: i16, dexterity: i16) -> f64 {
    let str_component = (strength as f64 + 1.0).ln(); // Natural log
    let dex_component = (dexterity as f64 + 1.0).ln(); // Natural log
    str_component + dex_component
}

/// Player statistics
#[derive(Debug, Clone, PartialEq)]
pub struct PlayerStats {
    pub strength: i16,
    pub intelligence: i16,
    pub dexterity: i16,
    pub charisma: i16,
    pub constitution: i16,
}

impl PlayerStats {
    /// Creates new player stats with the given values
    pub fn new(strength: i16, intelligence: i16, dexterity: i16, charisma: i16, constitution: i16) -> Self {
        Self {
            strength,
            intelligence,
            dexterity,
            charisma,
            constitution,
        }
    }

    /// Calculates the player's speed based on strength and dexterity
    pub fn speed(&self) -> f64 {
        calculate_speed(self.strength, self.dexterity)
    }

    /// Calculates total stat points
    pub fn total_stats(&self) -> i16 {
        self.strength + self.intelligence + self.dexterity + self.charisma + self.constitution
    }

    /// Checks if all stats are within valid ranges
    pub fn is_valid(&self) -> bool {
        self.strength >= 1 && self.strength <= 8
            && self.intelligence >= 3 && self.intelligence <= 5
            && self.dexterity >= 2 && self.dexterity <= 6
            && self.charisma >= 2 && self.charisma <= 4
            && self.constitution >= 2 && self.constitution <= 6
    }
}

/// Basic player skills
#[derive(Debug, Clone, PartialEq)]
pub struct PlayerSkills {
    pub walking: i8,
    pub swimming: i8,
    pub breathing: i8,
}

impl PlayerSkills {
    /// Creates new player skills with the given values
    pub fn new(walking: i8, swimming: i8, breathing: i8) -> Self {
        Self {
            walking,
            swimming,
            breathing,
        }
    }

    /// Checks if all skills are within valid ranges (3-5)
    pub fn is_valid(&self) -> bool {
        self.walking >= 3 && self.walking <= 5
            && self.swimming >= 3 && self.swimming <= 5
            && self.breathing >= 3 && self.breathing <= 5
    }

    /// Gets the average skill level
    pub fn average_skill(&self) -> f64 {
        (self.walking as f64 + self.swimming as f64 + self.breathing as f64) / 3.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_speed() {
        // Test basic speed calculation
        let speed1 = calculate_speed(5, 3);
        let speed2 = calculate_speed(3, 5);
        
        // Both should be positive
        assert!(speed1 > 0.0);
        assert!(speed2 > 0.0);
        
        // Higher stats should generally give higher speed
        let high_speed = calculate_speed(8, 6);
        let low_speed = calculate_speed(1, 2);
        assert!(high_speed > low_speed);
    }

    #[test]
    fn test_speed_calculation_properties() {
        // Test edge cases
        let min_speed = calculate_speed(1, 2);
        let max_speed = calculate_speed(8, 6);
        
        assert!(min_speed > 0.0);
        assert!(max_speed > min_speed);
        
        // Test that ln(1+1) = ln(2) ≈ 0.693
        let speed_1_1 = calculate_speed(1, 1);
        assert!((speed_1_1 - (2.0 * 2.0_f64.ln())).abs() < 0.001);
    }

    #[test]
    fn test_player_stats_creation() {
        let stats = PlayerStats::new(5, 4, 3, 2, 4);
        
        assert_eq!(stats.strength, 5);
        assert_eq!(stats.intelligence, 4);
        assert_eq!(stats.dexterity, 3);
        assert_eq!(stats.charisma, 2);
        assert_eq!(stats.constitution, 4);
    }

    #[test]
    fn test_player_stats_speed() {
        let stats = PlayerStats::new(5, 4, 3, 2, 4);
        let expected_speed = calculate_speed(5, 3);
        
        assert_eq!(stats.speed(), expected_speed);
    }

    #[test]
    fn test_player_stats_total() {
        let stats = PlayerStats::new(5, 4, 3, 2, 4);
        assert_eq!(stats.total_stats(), 18);
    }

    #[test]
    fn test_player_stats_validity() {
        // Valid stats
        let valid_stats = PlayerStats::new(5, 4, 3, 2, 4);
        assert!(valid_stats.is_valid());
        
        // Invalid strength (too low)
        let invalid_str = PlayerStats::new(0, 4, 3, 2, 4);
        assert!(!invalid_str.is_valid());
        
        // Invalid strength (too high)
        let invalid_str_high = PlayerStats::new(10, 4, 3, 2, 4);
        assert!(!invalid_str_high.is_valid());
        
        // Invalid intelligence (too low)
        let invalid_int = PlayerStats::new(5, 2, 3, 2, 4);
        assert!(!invalid_int.is_valid());
        
        // Invalid dexterity (too high)
        let invalid_dex = PlayerStats::new(5, 4, 7, 2, 4);
        assert!(!invalid_dex.is_valid());
    }

    #[test]
    fn test_player_skills_creation() {
        let skills = PlayerSkills::new(4, 3, 5);
        
        assert_eq!(skills.walking, 4);
        assert_eq!(skills.swimming, 3);
        assert_eq!(skills.breathing, 5);
    }

    #[test]
    fn test_player_skills_validity() {
        // Valid skills
        let valid_skills = PlayerSkills::new(4, 3, 5);
        assert!(valid_skills.is_valid());
        
        // Invalid walking (too low)
        let invalid_walk = PlayerSkills::new(2, 3, 5);
        assert!(!invalid_walk.is_valid());
        
        // Invalid swimming (too high)
        let invalid_swim = PlayerSkills::new(4, 6, 5);
        assert!(!invalid_swim.is_valid());
        
        // All at minimum valid values
        let min_skills = PlayerSkills::new(3, 3, 3);
        assert!(min_skills.is_valid());
        
        // All at maximum valid values
        let max_skills = PlayerSkills::new(5, 5, 5);
        assert!(max_skills.is_valid());
    }

    #[test]
    fn test_player_skills_average() {
        let skills = PlayerSkills::new(3, 4, 5);
        assert_eq!(skills.average_skill(), 4.0);
        
        let skills2 = PlayerSkills::new(5, 5, 5);
        assert_eq!(skills2.average_skill(), 5.0);
        
        let skills3 = PlayerSkills::new(3, 3, 3);
        assert_eq!(skills3.average_skill(), 3.0);
    }

    #[test]
    fn test_edge_case_stats() {
        // Test minimum valid stats
        let min_stats = PlayerStats::new(1, 3, 2, 2, 2);
        assert!(min_stats.is_valid());
        assert_eq!(min_stats.total_stats(), 10);
        
        // Test maximum valid stats
        let max_stats = PlayerStats::new(8, 5, 6, 4, 6);
        assert!(max_stats.is_valid());
        assert_eq!(max_stats.total_stats(), 29);
    }

    #[test]
    fn test_player_structs_clone_equality() {
        let stats1 = PlayerStats::new(5, 4, 3, 2, 4);
        let stats2 = stats1.clone();
        assert_eq!(stats1, stats2);
        
        let skills1 = PlayerSkills::new(4, 3, 5);
        let skills2 = skills1.clone();
        assert_eq!(skills1, skills2);
    }
}
