using System.Windows;
using Dungeon_Crawler_World.Floor.Floor1;

namespace Dungeon_Crawler_World.UI
{
  public partial class MainMenu : Window
  {
    public MainMenu()
    {
      //#32
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
      System.Text.StringBuilder sb = new System.Text.StringBuilder();
      sb.AppendLine(value: "New game started!");
      sb.AppendLine();
      sb.AppendLine(handler: $"Floor Time: {floorTime}:00");
      sb.AppendLine();
      sb.AppendLine(value: "Stats:");
      foreach (KeyValuePair<string, int> stat in stats)
      {
        sb.AppendLine(handler: $"- {char.ToUpper(c: stat.Key[index: 0]) + stat.Key[1..]}: {stat.Value}");
      }
      string statsMessage = sb.ToString();

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
