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
      Dictionary<string, int>? stats = StatsManager.LoadOrCreateStats();
      int floorTime = TimeManager.LoadOrCreateTime();

      // Create a user-friendly message showing the stats and time
      string statsMessage = "New game started!\n\nFloor Time: " + floorTime + ":00\n\nStats:";
      foreach (KeyValuePair<string, int> stat in stats)
      {
        statsMessage += $"\n- {char.ToUpper(c: stat.Key[index: 0]) + stat.Key[1..]}: {stat.Value}";
      }

      MessageBox.Show(messageBoxText: statsMessage,
          caption: "Game Started",
          button: MessageBoxButton.OK,
          icon: MessageBoxImage.Information);
    }

    private void GameSavesButton_Click(object sender, RoutedEventArgs e)
    {
      GameSavesWindow savesWindow = new GameSavesWindow();
      savesWindow.ShowDialog();
    }

    private void ExitButton_Click(object sender, RoutedEventArgs e)
    {
      // Close the application
      Application.Current.Shutdown();
    }
  }
}
