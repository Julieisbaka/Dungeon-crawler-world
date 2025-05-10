using System.IO;
using System.Text.Json;
using System;

namespace Dungeon_Crawler_World.Floor.Floor1
{
  public static class TimeManager
  {
    private static readonly Random random = new Random();
    private const string TIME_FILE_PATH = "Floor/Floor 1/time_data.json";

    // Load existing time data or create new time if not found
    public static int LoadOrCreateTime()
    {
      if (File.Exists(path: TIME_FILE_PATH))
      {
        try
        {
          string json = File.ReadAllText(path: TIME_FILE_PATH);
          TimeData? timeData = JsonSerializer.Deserialize<TimeData>(json: json);
          return timeData?.Hours ?? GenerateRandomTime();
        }
        catch (Exception ex)
        {
          LogError(message: $"Error loading time: {ex.Message}");
          return GenerateRandomTime();
        }
      }
      else
      {
        int hours = GenerateRandomTime();
        SaveTime(hours: hours);
        return hours;
      }
    }

    // Generate random time for a new game
    private static int GenerateRandomTime()
    {
      double u1 = 1.0 - random.NextDouble();
      double u2 = 1.0 - random.NextDouble();
      double randStdNormal = Math.Sqrt(d: -2.0 * Math.Log(d: u1)) * Math.Sin(a: 2.0 * Math.PI * u2); // Box-Muller transform
      double randNormal = 17 + randStdNormal;

      int randomTime = (int)Math.Round(a: randNormal);

      // Clamp to valid range (12-20)
      if (randomTime < 12)
        randomTime = 12;
      else if (randomTime > 20)
        randomTime = 20;

      return randomTime;
    }

    // Save time to a file
    public static void SaveTime(int hours)
    {
      try
      {
        EnsureDirectoryExists();
        string json = JsonSerializer.Serialize(value: new TimeData { Hours = hours });
        File.WriteAllText(path: TIME_FILE_PATH, contents: json);
      }
      catch (Exception ex)
      {
        LogError(message: $"Error saving time: {ex.Message}");
      }
    }

    // Ensure the directory for the time file exists
    private static void EnsureDirectoryExists()
    {
      string? directory = Path.GetDirectoryName(path: TIME_FILE_PATH);
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

  // Simple class to store time data
  public class TimeData
  {
    public int Hours { get; set; }
  }
}
