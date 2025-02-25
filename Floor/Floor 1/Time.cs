using System;

public class TimeGenerator
{
  private static readonly Random random = new Random();

  public static int GenerateRandomTime()
  {
    double mean = 15;
    double stdDev = 2;
    double u1 = 1.0 - random.NextDouble(); // uniform(0,1] random doubles
    double u2 = 1.0 - random.NextDouble();
    double randStdNormal = Math.Sqrt(-2.0 * Math.Log(u1)) * Math.Sin(2.0 * Math.PI * u2); // random normal(0,1)
    double randNormal = mean + stdDev * randStdNormal; // random normal(mean,stdDev)

    int randomTime = (int)Math.Round(randNormal);
    if (randomTime < 12)
      randomTime = 12;
    if (randomTime > 20)
      randomTime = 20;

    return randomTime;
  }
}
