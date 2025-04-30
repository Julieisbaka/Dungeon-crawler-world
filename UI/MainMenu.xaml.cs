using System.Windows;
using Dungeon_Crawler_World.Floor.Floor1;

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

    private void NewGameButton_Click(object sender, RoutedEventArgs e)
    {
      // Call stats and time file logic here
      StatsManager.LoadOrCreateStats();
      TimeManager.LoadOrCreateTime();

      MessageBox.Show(messageBoxText: "New game started! Stats and time files loaded.");
      //TODO: #33 Add logic to start the game here
    }

    private void GameSavesButton_Click(object sender, RoutedEventArgs e)
    {
      //TODO: #34 Add gamesaveswindow logic here
      GameSavesWindow savesWindow = new GameSavesWindow();
      savesWindow.ShowDialog();
    }
  }
}
