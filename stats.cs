using System;

namespace RandomNumberGenerator
{
    class Program
    {
        static void Main(string[] args)
        {
            // Create a new instance of Random
            Random random = new Random();
            
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