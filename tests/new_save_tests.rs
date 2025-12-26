use dungeon_crawler_world::{generate_floor_one_time, generate_stats};
use rand::thread_rng;

#[test]
fn test_real_time_generation() {
    let mut rng: rand::prelude::ThreadRng = thread_rng();
    let time: u32 = generate_floor_one_time(true, &mut rng);
    assert_eq!(time, 432_000, "Real time should be exactly 432000 seconds (5 days)");
}

#[test]
fn test_normal_time_generation_range() {
    let mut rng: rand::prelude::ThreadRng = thread_rng();
    // Run multiple times to check the range
    for _ in 0..100 {
        let time: u32 = generate_floor_one_time(false, &mut rng);
        // 12h = 43200, 20h = 72000
        assert!((43_200..=72_000).contains(&time), "Normal time should be between 12h and 20h in seconds, got {}", time);
    }
}

#[test]
fn test_stat_generation_ranges() {
    let mut rng: rand::prelude::ThreadRng = thread_rng();
    for _ in 0..100 {
        let (walking, swimming, breathing, strength, intelligence, dexterity, charisma, constitution) = generate_stats(&mut rng);
        assert!((3..=5).contains(&walking), "Walking out of range: {}", walking);
        assert!((3..=5).contains(&swimming), "Swimming out of range: {}", swimming);
        assert!((3..=5).contains(&breathing), "Breathing out of range: {}", breathing);
        assert!((1..=8).contains(&strength), "Strength out of range: {}", strength);
        assert!((3..=5).contains(&intelligence), "Intelligence out of range: {}", intelligence);
        assert!((2..=6).contains(&dexterity), "Dexterity out of range: {}", dexterity);
        assert!((2..=4).contains(&charisma), "Charisma out of range: {}", charisma);
        assert!((2..=6).contains(&constitution), "Constitution out of range: {}", constitution);
    }
}
