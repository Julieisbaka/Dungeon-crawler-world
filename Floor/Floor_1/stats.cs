namespace Dungeon_Crawler_World.Floor.Floor_1
{
  public static class StatsManager
  {
    private const string STATS_FILE_PATH = "Floor/Floor_1/stats_data.json";

    private static Dictionary<string, int> GenerateRandomStats() // Unused
    {
      Random random = new Random();
#pragma warning disable IDE0028 // Simplify collection initialization
      Dictionary<string, int> stats = new Dictionary<string, int>();
#pragma warning restore IDE0028

      string[] statNames = ["walking", "breathing", "blinking", "talking", "jumping", "writing", "reading"];

      // Generate 7 numbers between 3 and 5 (inclusive)
      foreach (string statName in statNames)
      {
        stats[key: statName] = random.Next(minValue: 3, maxValue: 6);
      }

      return stats;
    }
  }
}
