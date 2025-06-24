#include <random>
#include <cmath>
#include <algorithm>
#include <stdint.h>

// FFI-compatible function for random time generation
extern "C" {
    // Returns a random time (int, clamped 12-20)
    int32_t generate_random_time() {
        static std::random_device rd;
        static std::mt19937 gen(rd());
        static std::uniform_real_distribution<> dis(0.0, 1.0);

        double u1 = 1.0 - dis(gen);
        double u2 = 1.0 - dis(gen);
        double randStdNormal = std::sqrt(-2.0 * std::log(u1)) * std::sin(2.0 * M_PI * u2); // Box-Muller
        double randNormal = 17 + randStdNormal;

        int randomTime = static_cast<int>(std::round(randNormal));
        randomTime = std::clamp(randomTime, 12, 20);
        return randomTime;
    }
}
