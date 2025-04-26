using System.Windows;

namespace Dungeon_Crawler_World.UI
{
    public partial class MainMenu : Window
    {
        public MainMenu()
        {
            InitializeComponent();
        }

        private void PlayButton_Click(object sender, RoutedEventArgs e)
        {
            MessageBox.Show("The game is not ready to be played yet.",
                "Coming Soon",
                MessageBoxButton.OK,
                MessageBoxImage.Information);
        }

        private void SettingsButton_Click(object sender, RoutedEventArgs e)
        {
            var settingsPage = new SettingsPage();
            settingsPage.ShowDialog();
        }
    }
}
