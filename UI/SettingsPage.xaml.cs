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
        private Settings currentSettings;
        private bool isDisposed;

        public SettingsPage()
        {
            InitializeComponent();
            LoadSettings();
            DataContext = currentSettings;
        }

        private void LoadSettings()
        {
            try
            {
        EnsureConfigDirectoryExists();
                
                if (File.Exists(CONFIG_PATH))
                {
                    string jsonString = File.ReadAllText(CONFIG_PATH);
                    currentSettings = JsonSerializer.Deserialize<Settings>(jsonString) 
                        ?? CreateDefaultSettings();
                }
                else
                {
                    currentSettings = CreateDefaultSettings();
          SaveSettings(currentSettings); // Create initial config file
                }
            }
            catch (Exception ex)
            {
                Debug.WriteLine($"Error loading settings: {ex}");
                MessageBox.Show($"Error loading settings. Using defaults.\nError: {ex.Message}", 
                    "Settings Error", MessageBoxButton.OK, MessageBoxImage.Warning);
                currentSettings = CreateDefaultSettings();
            }
        }

        private static Settings CreateDefaultSettings()
        {
            return new Settings();
        }

        private static void EnsureConfigDirectoryExists()
        {
            if (!Directory.Exists(CONFIG_DIR))
            {
                Directory.CreateDirectory(CONFIG_DIR);
            }
        }

        private static void SaveSettings(Settings settings)
        {
            var options = new JsonSerializerOptions 
            { 
                WriteIndented = true 
            };
            string jsonString = JsonSerializer.Serialize(settings, options);
            File.WriteAllText(CONFIG_PATH, jsonString);
        }

        private void SaveButton_Click(object sender, RoutedEventArgs e)
        {
            try
            {
        SaveSettings(currentSettings);
                DialogResult = true;
                Close();
            }
            catch (Exception ex)
            {
                Debug.WriteLine($"Error saving settings: {ex}");
                MessageBox.Show($"Failed to save settings.\nError: {ex.Message}", 
                    "Save Error", MessageBoxButton.OK, MessageBoxImage.Error);
            }
        }

        private void CancelButton_Click(object sender, RoutedEventArgs e)
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
            GC.SuppressFinalize(this);
        }

        ~SettingsPage()
        {
            Dispose();
        }
    }
}