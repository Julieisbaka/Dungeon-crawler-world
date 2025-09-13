use rand::Rng;

const neighborhood_size: u16 = 1609;

/// Generates a random restroom distance within realistic bounds.
/// 
/// Restroom distance is approximately 400m (300-500m) based on typical
/// building layouts and accessibility requirements.
/// 
/// # Returns
/// 
/// A random value between 300 and 500 meters (inclusive).
/// 
/// # Examples
/// 
/// ```
/// let distance = restroom_distance();
/// assert!(distance >= 300 && distance <= 500);
/// ```
pub fn restroom_distance() -> u16 {
    let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
    (&mut rng).gen_range(300..=500)
}
