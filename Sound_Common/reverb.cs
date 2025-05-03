//#40
namespace Dungeon_Crawler_World.Sound_Common
{
  public class ReverbEffect
  {
    private const float MinDecayTime = 0.1f;

    public float ReverbLevel { get; set; } // Level of reverb effect (0.0 to 1.0)
    public float DecayTime { get; set; } // Time it takes for the reverb to decay (in seconds)
    public float RoomSize { get; set; } // Size of the virtual room (0.0 to 1.0)

    public ReverbEffect(float reverbLevel = 0.5f, float decayTime = 1.0f, float roomSize = 0.5f)
    {
      ReverbLevel = Math.Clamp(value: reverbLevel, min: 0.0f, max: 1.0f);
      DecayTime = Math.Max(val1: MinDecayTime, val2: decayTime); // Minimum decay time is 0.1 seconds
      RoomSize = Math.Clamp(value: roomSize, min: 0.0f, max: 1.0f);
    }

    public float ApplyReverb(float inputSample)
    {
      // Simulate a basic reverb effect by applying a simple decay and room size factor, this is a placeholder
      float reverbSample = inputSample * ReverbLevel * RoomSize;
      return reverbSample * (float)Math.Exp(d: -DecayTime);
    }

    public override string ToString()
    {
      return $"ReverbEffect [Level={ReverbLevel}, DecayTime={DecayTime}s, RoomSize={RoomSize}]";
    }
  }
}
