using System;

public class TimeGenerator
{
  private static readonly Random random = new Random();

  public static int GenerateRandomTimeInRange()
  {
    double u1 = 1.0 - random.NextDouble();
    double u2 = 1.0 - random.NextDouble();
    double randStdNormal = Math.Sqrt(-2.0 * Math.Log(u1)) * Math.Sin(2.0 * Math.PI * u2); // Using Box-Muller transform to generate a random normal distribution (mean=0, stdDev=1)
    double randNormal = 17 * randStdNormal; // random normal(mean,stdDev)

    int randomTime = (int)Math.Round(randNormal);

    if (randomTime <= 12)
    {
      randomTime = 12;
    }
    else if (randomTime >= 20)
    {
      randomTime = 20;
    }
    return randomTime;
  }
}
