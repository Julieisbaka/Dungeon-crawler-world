#include <random>
#include <cmath>
#include <algorithm>

namespace Dungeon_Crawler_World
{
    namespace Floor
    {
        namespace Floor_1
        {

            class floor1_time
            {
            public:
                static int GenerateRandomTime()
                {
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
            };

        }
    }
}
