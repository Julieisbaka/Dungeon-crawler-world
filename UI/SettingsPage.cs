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

    private static Settings CreateDefaultSettings()
    {
      return new Settings();
    }

    private static void EnsureConfigDirectoryExists()
    {
      if (!Directory.Exists(path: CONFIG_DIR))
      {
        Directory.CreateDirectory(path: CONFIG_DIR);
      }
    }

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

      // Save shader config to separate file
      /// <summary>
      /// The file path for the shader configuration JSON file.
      /// </summary>
      string shaderConfigPath = Path.Combine(path1: CONFIG_DIR, path2: "shader_config.json");
      string shaderConfigJson = JsonSerializer.Serialize(value: shaderConfig, options: options);
      File.WriteAllText(path: shaderConfigPath, contents: shaderConfigJson);
    }

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

    private void CancelButton_Click(object sender, RoutedEventArgs e)
    {
      DialogResult = false;
      Close();
    }

    private void ExitButton_Click(object sender, RoutedEventArgs e)
    {
      DialogResult = false;
      Close();
    }

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
