namespace Dungeon_Crawler_World.Floor.Floor1
{
  public static class TimeManager
  {
    private static readonly Random random = new Random();

    public static void LoadOrCreateTime()
    {
      double u1 = 1.0 - random.NextDouble();
      double u2 = 1.0 - random.NextDouble();
      double randStdNormal = Math.Sqrt(-2.0 * Math.Log(u1)) * Math.Sin(2.0 * Math.PI * u2); // Box-Muller transform
      double randNormal = 17 * randStdNormal;

      int randomTime = (int)Math.Round(randNormal);

      if (randomTime <= 12)
      {
        randomTime = 12;
      }
      else if (randomTime >= 20)
      {
        randomTime = 20;
      }

      // Add logic to save/load time as needed
    }
  }
}
