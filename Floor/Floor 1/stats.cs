namespace RandomNumberGenerator
{
    class Stats
    {
        static void ProcessStats(string[] args)
        {
            Random random = new();

            // Generate 8 numbers between 3 and 5 (inclusive)            
            for (int i = 0; i < 8; i++)
            {
                // Next(minValue, maxValue) returns a random number >= minValue and < maxValue
                // So to include 5, we use maxValue of 6
                int randomNumber = random.Next(3, 6);
            }
        }
    }
}