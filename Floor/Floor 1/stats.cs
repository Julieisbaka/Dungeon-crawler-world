using System.IO;
using System.Text.Json;
using System;

namespace Dungeon_Crawler_World.Floor.Floor1
{
  public static class StatsManager
  {
    private const string STATS_FILE_PATH = "Floor/Floor 1/stats_data.json";

    // Load existing stats or create new ones if not found
    public static Dictionary<string, int> LoadOrCreateStats()
    {
      if (File.Exists(path: STATS_FILE_PATH))
      {
        try
        {
          string json = File.ReadAllText(path: STATS_FILE_PATH);
          Dictionary<string, int>? stats = JsonSerializer.Deserialize<Dictionary<string, int>>(json: json);
          return stats ?? GenerateRandomStats();
        }
        catch (Exception ex)
        {
          LogError(message: $"Error loading stats: {ex.Message}");
          return GenerateRandomStats();
        }
      }
      else
      {
        Dictionary<string, int> stats = GenerateRandomStats();
        SaveStats(stats: stats);
        return stats;
      }
    }

    // Generate random stats for a new game
    private static Dictionary<string, int> GenerateRandomStats()
    {
      Random random = new Random();
      Dictionary<string, int> stats = new Dictionary<string, int>();

      string[] statNames = { "walking", "breathing", "blinking", "talking", "jumping", "writing", "reading", "climbing" };

      foreach (string statName in statNames)
      {
        stats[key: statName] = random.Next(minValue: 3, maxValue: 6);
      }

      return stats;
    }

    // Save stats to a file
    public static void SaveStats(Dictionary<string, int> stats)
    {
      try
      {
        EnsureDirectoryExists();
        string json = JsonSerializer.Serialize(value: stats);
        File.WriteAllText(path: STATS_FILE_PATH, contents: json);
      }
      catch (Exception ex)
      {
        LogError(message: $"Error saving stats: {ex.Message}");
      }
    }

    // Ensure the directory for the stats file exists
    private static void EnsureDirectoryExists()
    {
      string? directory = Path.GetDirectoryName(path: STATS_FILE_PATH);
      if (!string.IsNullOrEmpty(value: directory))
      {
        Directory.CreateDirectory(path: directory);
      }
    }

    // Log errors to the console
    private static void LogError(string message)
    {
      Console.WriteLine(value: $"[ERROR] {message}");
    }
  }
}
