#include <random>
#include <stdint.h>

// FFI-compatible stat count and names
extern "C" {
    // Number of stats
    static const int STAT_COUNT = 7;
    // Stat names in order
    static const char* STAT_NAMES[STAT_COUNT] = {
        "walking",
        "breathing",
        "blinking",
        "talking",
        "jumping",
        "writing",
        "reading"
    };

    // Fills the provided buffer with random stat values (range 3-5)
    // buffer must be at least STAT_COUNT elements long
    void generate_random_stats(int32_t* buffer) {
        std::random_device rd;
        std::mt19937 gen(rd());
        std::uniform_int_distribution<> dis(3, 5);
        for (int i = 0; i < STAT_COUNT; ++i) {
            buffer[i] = dis(gen);
        }
    }

    // Returns the number of stats
    int get_stat_count() {
        return STAT_COUNT;
    }

    // Returns the stat name at the given index (0-based)
    const char* get_stat_name(int index) {
        if (index < 0 || index >= STAT_COUNT) return nullptr;
        return STAT_NAMES[index];
    }
}