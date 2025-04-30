namespace Dungeon_Crawler_World.Floor.Floor1
{
  public static class StatsManager
  {
    public static void LoadOrCreateStats()
    {
      Random random = new Random();

      string[] statNames = { "walking", "breathing", "blinking", "talking", "jumping", "writing", "reading", "climbing" };
      int[] statValues = new int[8];

      // Generate 8 numbers between 3 and 5 (inclusive)
      for (int i = 0; i < 8; i++)
      {
        statValues[i] = random.Next(3, 6);
      }

      // You can add logic here to save/load stats as needed
    }
  }
}
