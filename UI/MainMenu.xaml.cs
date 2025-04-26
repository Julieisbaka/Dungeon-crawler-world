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
      MessageBox.Show(messageBoxText: "The game is not ready to be played yet.",
          caption: "Coming Soon",
          button: MessageBoxButton.OK,
          icon: MessageBoxImage.Information);
    }

    private void SettingsButton_Click(object sender, RoutedEventArgs e)
    {
      SettingsPage? settingsPage = new SettingsPage();
      settingsPage.ShowDialog();
    }
  }
}
