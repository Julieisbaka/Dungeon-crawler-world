// #52
using System.ComponentModel;
using System.Runtime.CompilerServices;

namespace Dungeon_Crawler_World.UI
{
  public class Settings : INotifyPropertyChanged
  {
    // General application settings
    private bool _musicEnabled = true;
    public bool MusicEnabled
    {
      get => _musicEnabled;
      set => SetProperty(field: ref _musicEnabled, value: value);
    }

    private bool _soundEffectsEnabled = true;
    public bool SoundEffectsEnabled
    {
      get => _soundEffectsEnabled;
      set => SetProperty(field: ref _soundEffectsEnabled, value: value);
    }

    private int _volume = 75;
    public int Volume
    {
      get => _volume;
      set => SetProperty(field: ref _volume, value: value);
    }

    private int _fps = 60;
    public int FPS
    {
      get => _fps;
      set => SetProperty(field: ref _fps, value: value);
    }

    // Display settings
    private bool _fullscreen;
    public bool Fullscreen
    {
      get => _fullscreen;
      set => SetProperty(field: ref _fullscreen, value: value);
    }

    private int _windowWidth = 1280;
    public int WindowWidth
    {
      get => _windowWidth;
      set => SetProperty(field: ref _windowWidth, value: value);
    }

    private int _windowHeight = 720;
    public int WindowHeight
    {
      get => _windowHeight;
      set => SetProperty(field: ref _windowHeight, value: value);
    }

    // Shader settings
    private bool _shadersEnabled = true;
    public bool ShadersEnabled
    {
      get => _shadersEnabled;
      set => SetProperty(field: ref _shadersEnabled, value: value);
    }

    private float _bloomIntensity = 0.5f;
    public float BloomIntensity
    {
      get => _bloomIntensity;
      set => SetProperty(field: ref _bloomIntensity, value: value);
    }

    private float _bloomThreshold = 0.8f;
    public float BloomThreshold
    {
      get => _bloomThreshold;
      set => SetProperty(field: ref _bloomThreshold, value: value);
    }

    private bool _ambientOcclusionEnabled = true;
    public bool AmbientOcclusionEnabled
    {
      get => _ambientOcclusionEnabled;
      set => SetProperty(field: ref _ambientOcclusionEnabled, value: value);
    }

    private float _ambientOcclusionStrength = 1.0f;
    public float AmbientOcclusionStrength
    {
      get => _ambientOcclusionStrength;
      set => SetProperty(field: ref _ambientOcclusionStrength, value: value);
    }

    private bool _shadowsEnabled = true;
    public bool ShadowsEnabled
    {
      get => _shadowsEnabled;
      set => SetProperty(field: ref _shadowsEnabled, value: value);
    }

    private int _shadowResolution = 1024;
    public int ShadowResolution
    {
      get => _shadowResolution;
      set => SetProperty(field: ref _shadowResolution, value: value);
    }

    private float _shadowSoftness = 2.0f;
    public float ShadowSoftness
    {
      get => _shadowSoftness;
      set => SetProperty(field: ref _shadowSoftness, value: value);
    }

    // Shader schema properties
    private int _fogLevel = 2;
    public int FogLevel
    {
      get => _fogLevel;
      set => SetProperty(field: ref _fogLevel, value: value);
    }

    private int _lightingLevel = 2;
    public int LightingLevel
    {
      get => _lightingLevel;
      set => SetProperty(field: ref _lightingLevel, value: value);
    }

    private bool _physicalSound = true;
    public bool PhysicalSound
    {
      get => _physicalSound;
      set => SetProperty(field: ref _physicalSound, value: value);
    }

    // INotifyPropertyChanged implementation
    public event PropertyChangedEventHandler? PropertyChanged;

    protected void OnPropertyChanged([CallerMemberName] string? propertyName = null)
    {
      PropertyChanged?.Invoke(sender: this, e: new PropertyChangedEventArgs(propertyName: propertyName));
    }

    // Note: The Equals method is used to compare the current and new values.
    // For reference types, ensure that Equals is overridden appropriately to avoid performance issues.
    // protected bool SetProperty<T>(ref T field, T value, [CallerMemberName] string? propertyName = null)
    /// Sets the value of a property and raises the PropertyChanged event if the value changes.
    /// </summary>
    /// <typeparam name="T">The type of the property.</typeparam>
    /// <param name="field">A reference to the field storing the property's value.</param>
    /// <param name="value">The new value to set.</param>
    /// <param name="propertyName">The name of the property (automatically provided by the compiler).</param>
    /// <returns>True if the value was changed; otherwise, false.</returns>
    protected bool SetProperty<T>(ref T field, T value, [CallerMemberName] string? propertyName = null)
    {
      if (Equals(objA: field, objB: value))
      {
        return false;
      }

      field = value;
      OnPropertyChanged(propertyName: propertyName);
      return true;
    }
  }
}
