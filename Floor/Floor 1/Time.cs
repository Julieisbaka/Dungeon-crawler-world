using System.IO;
using System.Text.Json;

namespace Dungeon_Crawler_World.Floor.Floor1
{
  public static class TimeManager
  {
    private static readonly Random random = new Random();
    private const string TIME_FILE_PATH = "Floor/Floor 1/time_data.json";

    public static int LoadOrCreateTime()
    {
      // Try to load existing time data first
      if (File.Exists(path: TIME_FILE_PATH))
      {
        try
        {
          string json = File.ReadAllText(path: TIME_FILE_PATH);
          TimeData? timeData = JsonSerializer.Deserialize<TimeData>(json: json);
          return timeData?.Hours ?? GenerateRandomTime();
        }
        catch (Exception)
        {
          // If any error occurs during loading, generate new time
          return GenerateRandomTime();
        }
      }
      else
      {
        // No existing time data, generate and save new time
        int hours = GenerateRandomTime();
        SaveTime(hours: hours);
        return hours;
      }
    }

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

    public static void SaveTime(int hours)
    {
      // Ensure directory exists
      string? directory = Path.GetDirectoryName(path: TIME_FILE_PATH);
      if (!string.IsNullOrEmpty(value: directory))
      {
        Directory.CreateDirectory(path: directory);
      }

      // Create and save time data
      TimeData timeData = new TimeData() { Hours = hours };
      string json = JsonSerializer.Serialize(value: timeData);
      File.WriteAllText(path: TIME_FILE_PATH, contents: json);
    }
  }

  // Simple class to store time data
  public class TimeData
  {
    public int Hours { get; set; }
  }
}
