use rand::Rng;

// Every swuare consists of 4 neighorhoods. The interior of these neighborhoods will not be a grid.
const neighborhood_size: u16 = 1609;

/// Restroom distance is approximately 400m (300-500m)
/// Returns a random value between 300 and 500 (inclusive).
pub fn restroom_distance() -> u16 {
    let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
    (&mut rng).gen_range(300..=500)
}
