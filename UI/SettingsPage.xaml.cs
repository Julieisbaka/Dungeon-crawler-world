namespace DungeonCrawlerWorld.UI
{
    public partial class SettingsPage : UserControl
    {
        private static readonly string ConfigFilePath =
            Path.Combine(AppDomain.CurrentDomain.BaseDirectory, "Shaders", "config.json");

        public SettingsPage()
        {
            InitializeComponent();
            LoadSettings();
        }

        private void LoadSettings()
        {
            if (!File.Exists(ConfigFilePath)) return;
            var json = File.ReadAllText(ConfigFilePath);
            var cfg = JsonSerializer.Deserialize<ShaderConfig>(json);
            if (cfg == null) return;

            foreach (ComboBoxItem it in FogComboBox.Items)
                if ((int)it.Tag == cfg.Fog) FogComboBox.SelectedItem = it;

            foreach (ComboBoxItem it in LightingComboBox.Items)
                if ((int)it.Tag == cfg.Lighting) LightingComboBox.SelectedItem = it;

            SoundCheckBox.IsChecked = cfg.Sound;
        }

        private void SaveButton_Click(object sender, RoutedEventArgs e)
        {
            var cfg = new ShaderConfig
            {
                Fog = (int)((ComboBoxItem)FogComboBox.SelectedItem).Tag,
                Lighting = (int)((ComboBoxItem)LightingComboBox.SelectedItem).Tag,
                Sound = SoundCheckBox.IsChecked == true
            };

            var opts = new JsonSerializerOptions { WriteIndented = true };
            var outJson = JsonSerializer.Serialize(cfg, opts);
            File.WriteAllText(ConfigFilePath, outJson);

            MessageBox.Show("Settings saved.", "Info", MessageBoxButton.OK, MessageBoxImage.Information);
        }
    }

    public class ShaderConfig
    {
        public int Fog { get; set; }
        public int Lighting { get; set; }
        public bool Sound { get; set; }
    }
}