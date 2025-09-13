use rand::Rng;

const neighborhood_size: u16 = 1609;

/// Restroom distance is approximately 400m (300-500m)
/// Returns a random value between 300 and 500 (inclusive).
pub fn restroom_distance() -> u16 {
    let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
    (&mut rng).gen_range(300..=500)
}

/// Restroom distance with seeded RNG for deterministic testing
pub fn restroom_distance_seeded<R: Rng>(rng: &mut R) -> u16 {
    rng.gen_range(300..=500)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    #[test]
    fn test_restroom_distance_bounds() {
        // Test with multiple random calls to ensure bounds are respected
        for _ in 0..1000 {
            let distance = restroom_distance();
            assert!(distance >= 300, "Distance {} is below minimum 300", distance);
            assert!(distance <= 500, "Distance {} is above maximum 500", distance);
        }
    }

    #[test]
    fn test_restroom_distance_deterministic() {
        // Test with seeded RNG for reproducible results
        let mut rng = ChaCha8Rng::seed_from_u64(12345);
        let distance1 = restroom_distance_seeded(&mut rng);
        
        // Reset RNG with same seed
        let mut rng = ChaCha8Rng::seed_from_u64(12345);
        let distance2 = restroom_distance_seeded(&mut rng);
        
        assert_eq!(distance1, distance2, "Seeded RNG should produce deterministic results");
        assert!(distance1 >= 300 && distance1 <= 500, "Seeded distance should be in valid range");
    }

    #[test]
    fn test_restroom_distance_distribution() {
        // Test that the function produces a reasonable distribution
        let mut rng = ChaCha8Rng::seed_from_u64(67890);
        let mut results = Vec::new();
        
        for _ in 0..1000 {
            results.push(restroom_distance_seeded(&mut rng));
        }
        
        // Check that we get values across the range
        let min_found = *results.iter().min().unwrap();
        let max_found = *results.iter().max().unwrap();
        
        assert!(min_found >= 300, "Minimum value should be >= 300");
        assert!(max_found <= 500, "Maximum value should be <= 500");
        
        // With 1000 samples, we should see values near both ends
        assert!(min_found <= 320, "Should see values near the minimum");
        assert!(max_found >= 480, "Should see values near the maximum");
    }

    #[test]
    fn test_neighborhood_size_constant() {
        // Verify the neighborhood size constant
        assert_eq!(neighborhood_size, 1609);
    }
}
