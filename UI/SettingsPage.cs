using System.Windows;
using System.Text.Json;
using System.IO;
using System.Diagnostics;

namespace Dungeon_Crawler_World.UI
{
  public partial class SettingsPage : Window, IDisposable
  {
    private const string CONFIG_PATH = @"Shaders\config.json";
    private const string CONFIG_DIR = @"Shaders";
    private Settings currentSettings = CreateDefaultSettings();
    private bool isDisposed;

    public SettingsPage()
    {
      //#32
      InitializeComponent();
      LoadSettings();
      DataContext = currentSettings;
    }

    /// <summary>
    /// Loads the settings from the configuration file or creates default settings if the file does not exist.
    /// </summary>
    private void LoadSettings()
    {
      try
      {
        EnsureConfigDirectoryExists();

        if (File.Exists(path: CONFIG_PATH))
        {
          string jsonString = File.ReadAllText(path: CONFIG_PATH);
          currentSettings = JsonSerializer.Deserialize<Settings>(json: jsonString)
              ?? CreateDefaultSettings();
        }
        else
        {
          currentSettings = CreateDefaultSettings();
          SaveSettings(settings: currentSettings); // Create initial config file
        }
      }
      catch (Exception ex)
      {
        Debug.WriteLine(message: $"Error loading settings: {ex}");
        MessageBox.Show(messageBoxText: $"Error loading settings. Using defaults.\nError: {ex.Message}",
            caption: "Settings Error", button: MessageBoxButton.OK, icon: MessageBoxImage.Warning);
        currentSettings = CreateDefaultSettings();
      }
    }

    /// <summary>
    /// Creates default settings for the application.
    /// </summary>
    /// <returns>A new instance of the Settings class with default values.</returns>
    private static Settings CreateDefaultSettings()
    {
      return new Settings();
    }

    /// <summary>
    /// Ensures that the configuration directory exists. If it does not exist, it will be created.
    /// </summary>
    private static void EnsureConfigDirectoryExists()
    {
      if (!Directory.Exists(path: CONFIG_DIR))
      {
        Directory.CreateDirectory(path: CONFIG_DIR);
      }
    }

    /// <summary>
    /// Saves the settings to the configuration file and validates the shader configuration against the schema.
    /// </summary>
    /// <param name="settings">The settings to save.</param>
    private static void SaveSettings(Settings settings)
    {
      JsonSerializerOptions? options = new JsonSerializerOptions
      {
        WriteIndented = true
      };

      // Save all settings to config.json
      string jsonString = JsonSerializer.Serialize(value: settings, options: options);
      File.WriteAllText(path: CONFIG_PATH, contents: jsonString);

      // Also save shader-specific config according to the schema
      var shaderConfig = new
      {
        fog = settings.FogLevel,
        lighting = settings.LightingLevel,
        sound = settings.PhysicalSound
      };

      // Validate shader config against schema
      string error;
      if (!SettingsPage.ValidateShaderConfig(shaderConfig: shaderConfig, error: out error))
      {
        Debug.WriteLine(message: $"Shader config validation failed: {error}");
        MessageBox.Show(messageBoxText: $"Shader config validation failed:\n{error}", caption: "Validation Error", button: MessageBoxButton.OK, icon: MessageBoxImage.Error);
        return;
      }

      // Save shader config to separate file
      /// <summary>
      /// The file path for the shader configuration JSON file.
      /// </summary>
      string shaderConfigPath = Path.Combine(path1: CONFIG_DIR, path2: "shader_config.json");
      string shaderConfigJson = JsonSerializer.Serialize(value: shaderConfig, options: options);
      File.WriteAllText(path: shaderConfigPath, contents: shaderConfigJson);
    }

    /// <summary>
    /// Handles the click event for the Save button. Saves the current settings and closes the window.
    /// </summary>
    private void SaveButton_Click(object sender, RoutedEventArgs e)
    {
      try
      {
        SaveSettings(settings: currentSettings);
        MessageBox.Show(messageBoxText: "Settings saved successfully!", caption: "Settings", button: MessageBoxButton.OK, icon: MessageBoxImage.Information);
        DialogResult = true;
        Close();
      }
      catch (Exception ex)
      {
        Debug.WriteLine(message: $"Error saving settings: {ex}");
        MessageBox.Show(messageBoxText: $"Failed to save settings.\nError: {ex.Message}",
            caption: "Save Error", button: MessageBoxButton.OK, icon: MessageBoxImage.Error);
      }
    }

    /// <summary>
    /// Handles the click event for the Cancel button. Closes the window without saving changes.
    /// </summary>
    private void CancelButton_Click(object sender, RoutedEventArgs e)
    {
      DialogResult = false;
      Close();
    }

    /// <summary>
    /// Handles the click event for the Exit button. Closes the window without saving changes.
    /// </summary>
    private void ExitButton_Click(object sender, RoutedEventArgs e)
    {
      DialogResult = false;
      Close();
    }

    /// <summary>
    /// Disposes of the resources used by the SettingsPage.
    /// </summary>
    public void Dispose()
    {
      if (!isDisposed)
      {
        // Clean up any resources here
        isDisposed = true;
      }
      GC.SuppressFinalize(obj: this);
    }

    ~SettingsPage()
    {
      Dispose();
    }
  }
}
