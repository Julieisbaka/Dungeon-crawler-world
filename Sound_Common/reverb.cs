namespace Dungeon_Crawler_World.Sound_Common
{
    public class ReverbEffect
    {
        public float ReverbLevel { get; set; } // Level of reverb effect (0.0 to 1.0)
        public float DecayTime { get; set; } // Time it takes for the reverb to decay (in seconds)
        public float RoomSize { get; set; } // Size of the virtual room (0.0 to 1.0)

        public ReverbEffect(float reverbLevel = 0.5f, float decayTime = 1.0f, float roomSize = 0.5f)
        {
            ReverbLevel = Math.Clamp(reverbLevel, 0.0f, 1.0f);
            DecayTime = Math.Max(0.1f, decayTime); // Minimum decay time is 0.1 seconds
            RoomSize = Math.Clamp(roomSize, 0.0f, 1.0f);
        }

        public float ApplyReverb(float inputSample)
        {
            // Simulate a basic reverb effect by applying a simple decay and room size factor
            float reverbSample = inputSample * ReverbLevel * RoomSize;
            return reverbSample * (float)Math.Exp(-DecayTime);
        }

        public override string ToString()
        {
            return $"ReverbEffect [Level={ReverbLevel}, DecayTime={DecayTime}s, RoomSize={RoomSize}]";
        }
    }
}