namespace Dungeon_Crawler_World.Floor.Floor_1
{
  public static class TimeManager
  {
    private static readonly Random random = new Random();
    private const string TIME_FILE_PATH = "Floor/Floor_1/time_data.json";

    private static int GenerateRandomTime() // Unused
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
  }
}
