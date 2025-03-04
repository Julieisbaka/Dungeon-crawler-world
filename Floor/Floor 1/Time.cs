using System;

public class TimeGenerator
{
  private static readonly Random random = new Random();

  public static int GenerateRandomTimeInRange()
  {
    const double mean = 15;
    const double stdDev = 2;
    double u1 = 1.0 - random.NextDouble(); // ensures the random value is within the range (0, 1]
    double u2 = 1.0 - random.NextDouble();
    // Using Box-Muller transform to generate a random normal distribution (mean=0, stdDev=1)
    double randStdNormal = Math.Sqrt(-2.0 * Math.Log(u1)) * Math.Sin(2.0 * Math.PI * u2);
    double randNormal = mean + stdDev * randStdNormal; // random normal(mean,stdDev)

    if (randomTime < 12)
    {
      randomTime = 12;
    }
    if (randomTime > 20)
    {
      randomTime = 20;
    }
      randomTime = 20;

    return randomTime;
  }
}
