using System.IO;
using System.Text.Json;

namespace Dungeon_Crawler_World.Floor.Floor1
{
  public static class StatsManager
  {
    private const string STATS_FILE_PATH = "Floor/Floor 1/stats_data.json";

    public static Dictionary<string, int> LoadOrCreateStats()
    {
      // Try to load existing stats first
      if (File.Exists(path: STATS_FILE_PATH))
      {
        try
        {
          string json = File.ReadAllText(path: STATS_FILE_PATH);
          Dictionary<string, int>? stats = JsonSerializer.Deserialize<Dictionary<string, int>>(json: json);
          return stats ?? GenerateRandomStats();
        }
        catch (Exception)
        {
          // If any error occurs during loading, generate new stats
          return GenerateRandomStats();
        }
      }
      else
      {
        // No existing stats, generate and save new ones
        Dictionary<string, int> stats = GenerateRandomStats();
        SaveStats(stats: stats);
        return stats;
      }
    }

    private static Dictionary<string, int> GenerateRandomStats()
    {
      Random random = new Random();
      Dictionary<string, int> stats = new Dictionary<string, int>();

      string[] statNames = { "walking", "breathing", "blinking", "talking", "jumping", "writing", "reading", "climbing" };

      // Generate 8 numbers between 3 and 5 (inclusive)
      foreach (string statName in statNames)
      {
        stats[key: statName] = random.Next(minValue: 3, maxValue: 6);
      }

      return stats;
    }

    public static void SaveStats(Dictionary<string, int> stats)
    {
      // Ensure directory exists
      string? directory = Path.GetDirectoryName(path: STATS_FILE_PATH);
      if (!string.IsNullOrEmpty(value: directory))
      {
        Directory.CreateDirectory(path: directory);
      }

      // Serialize and save stats
      string json = JsonSerializer.Serialize(value: stats);
      File.WriteAllText(path: STATS_FILE_PATH, contents: json);
    }

    public static void UpdateAIBehaviorBasedOnStats(Dictionary<string, int> stats)
    {
      // Example logic to influence AI behavior based on character statistics
      int walkingStat = stats.ContainsKey("walking") ? stats["walking"] : 0;
      int breathingStat = stats.ContainsKey("breathing") ? stats["breathing"] : 0;

      // Adjust AI behavior based on stats
      if (walkingStat > 4)
      {
        // AI character moves faster
      }

      if (breathingStat < 3)
      {
        // AI character gets tired more quickly
      }
    }
  }
}
