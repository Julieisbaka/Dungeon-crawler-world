#include <random>
#include <map>
#include <string>
#include <vector>

namespace Dungeon_Crawler_World
{
    namespace Floor
    {
        namespace Floor_1
        {
            class StatsManager
            {
            public:
                static std::map<std::string, int> GenerateRandomStats()
                {
                    std::random_device rd;
                    std::mt19937 gen(rd());
                    std::uniform_int_distribution<> dis(3, 5);

                    std::vector<std::string> statNames = {"walking", "breathing", "blinking", "talking", "jumping", "writing", "reading"};
                    std::map<std::string, int> stats;

                    for (const auto &statName : statNames)
                    {
                        stats[statName] = dis(gen);
                    }

                    return stats;
                }
            };
        }
    }
}