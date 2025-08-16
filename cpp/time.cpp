#include <random>
#define _USE_MATH_DEFINES
#include <cmath>
#ifndef M_PI
#define M_PI 3.14159265358979323846
#endif
#include <algorithm>
#include <stdint.h>

// FFI-compatible function for random time generation
extern "C" {
// Returns a random time with specified min, max, and mean
int32_t random_time_range(int32_t min, int32_t max, double mean) {
  // Input validation
  if (min > max) {
    // Swap values if min > max
    int32_t temp = min;
    min = max;
    max = temp;
  }

  // Check for invalid mean values
  if (!std::isfinite(mean)) {
    // Default to midpoint if mean is invalid
    mean = static_cast<double>(min + max) / 2.0;
  }

  static std::random_device rd;
  static std::mt19937 gen(
      rd()); // std::mersenne_twister_engine<unsigned int, 32Ui64, 624Ui64,
             // 397Ui64, 31Ui64, 2567483615U, 11Ui64, 4294967295U, 7Ui64,
             // 2636928640U, 15Ui64, 4022730752U, 18Ui64, 1812433253U>
  static std::uniform_real_distribution<> dis(0.0, 1.0);

  double u1 = 1.0 - dis(gen);
  double u2 = 1.0 - dis(gen);
  double randStdNormal =
      std::sqrt(-2.0 * std::log(u1)) * std::sin(2.0 * M_PI * u2); // Box-Muller
  double randNormal = mean + randStdNormal;

  int randomTime = static_cast<int>(std::round(randNormal));
  randomTime = std::clamp(randomTime, min, max);
  return randomTime;
}
}
