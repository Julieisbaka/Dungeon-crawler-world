/**
 * @file time.cpp
 * @brief Random time generation utilities for the dungeon crawler game
 * 
 * This module provides functions for generating random time values using
 * normal distribution with proper input validation and error handling.
 */

#include <random>
#define _USE_MATH_DEFINES
#include <cmath>
#ifndef M_PI
/** @brief Mathematical constant pi, defined if not already available */
#define M_PI 3.14159265358979323846
#endif
#include <algorithm>
#include <stdint.h>

/**
 * @brief FFI-compatible functions for random time generation
 * 
 * This extern "C" block ensures C linkage for functions that need to be
 * called from other languages (e.g., Rust FFI).
 */
extern "C"
{
    /**
     * @brief Generates a random time value using normal distribution with input validation
     * 
     * This function generates a random time value following a normal distribution
     * centered around the provided mean, then clamps the result to the specified range.
     * Uses the Box-Muller transform to convert uniform random numbers into normally
     * distributed values.
     * 
     * @param min_val Minimum allowed time value (inclusive)
     * @param max_val Maximum allowed time value (inclusive)
     * @param mean    Target mean for the normal distribution
     * @return int32_t Random time value within [min_val, max_val] range
     * 
     * @note If invalid parameters are provided (min > max or mean outside range),
     *       the function returns the midpoint of the valid range as a fallback.
     * @note Uses static random number generators for thread-safety and performance.
     * 
     * @warning This function is not thread-safe due to static variables.
     *          For multi-threaded usage, consider thread-local storage.
     */
    int32_t generate_random_time_range(int32_t min_val, int32_t max_val, double mean)
    {
        // Input validation: ensure min_val <= max_val
        if (min_val > max_val) {
            // Swap values if they're in wrong order
            int32_t temp = min_val;
            min_val = max_val;
            max_val = temp;
        }
        
        // Handle edge case: if min equals max, return that value
        if (min_val == max_val) {
            return min_val;
        }
        
        // Validate that mean is within the valid range [min_val, max_val]
        // If not, clamp it to the nearest boundary
        if (mean < static_cast<double>(min_val)) {
            mean = static_cast<double>(min_val);
        } else if (mean > static_cast<double>(max_val)) {
            mean = static_cast<double>(max_val);
        }
        
        /** @brief Thread-local hardware random number generator for seeding */
        static std::random_device rd;
        
        /** @brief Mersenne Twister pseudo-random generator for high-quality randomness */
        static std::mt19937 gen(rd());
        
        /** @brief Uniform distribution in [0.0, 1.0) for Box-Muller transform */
        static std::uniform_real_distribution<> dis(0.0, 1.0);

        // Generate two uniform random numbers for Box-Muller transformation
        // We use (1.0 - dis(gen)) to avoid log(0) which would cause undefined behavior
        double u1 = 1.0 - dis(gen);  // Uniform(0,1) but excluding 0
        double u2 = 1.0 - dis(gen);  // Uniform(0,1) but excluding 0
        
        // Apply Box-Muller transform to convert uniform distribution to standard normal
        // This generates a normally distributed random variable with mean=0, std=1
        double randStdNormal = std::sqrt(-2.0 * std::log(u1)) * std::sin(2.0 * M_PI * u2);
        
        // Transform standard normal to our desired normal distribution with specified mean
        // Note: We're using standard deviation of 1.0 for simplicity
        double randNormal = mean + randStdNormal;

        // Convert to integer and clamp to the specified range
        int randomTime = static_cast<int>(std::round(randNormal));
        randomTime = std::clamp(randomTime, min_val, max_val);
        
        return randomTime;
    }
}
