using System.Windows;
using Dungeon_Crawler_World.Floor.Floor1;

namespace Dungeon_Crawler_World.UI
{
  public partial class MainMenu : Window
  {
    public MainMenu()
    {
      // Initialize the main menu components
      InitializeComponent();
    }

    private void PlayButton_Click(object sender, RoutedEventArgs e)
    {
      // Display a message indicating that the game is not ready to be played yet
      MessageBox.Show(messageBoxText: "The game is not ready to be played yet.",
          caption: "Coming Soon",
          button: MessageBoxButton.OK,
          icon: MessageBoxImage.Information);
    }

    private void SettingsButton_Click(object sender, RoutedEventArgs e)
    {
      // Open the settings page
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

      // Display the stats and time message
      MessageBox.Show(messageBoxText: statsMessage,
          caption: "Game Started",
          button: MessageBoxButton.OK,
          icon: MessageBoxImage.Information);
    }

    private void GameSavesButton_Click(object sender, RoutedEventArgs e)
    {
      // Open the game saves window
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
