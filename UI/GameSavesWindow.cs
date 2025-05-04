using System.Windows;

namespace Dungeon_Crawler_World.UI
{
  public partial class GameSavesWindow : Window
  {
    public GameSavesWindow()
    {
      InitializeComponent();
      Title = "Game Saves";
      Width = 600;
      Height = 400;
    }

    private void InitializeComponent()
    {
      // Simple placeholder window
      Content = new System.Windows.Controls.TextBlock
      {
        Text = "Game Saves functionality will be implemented in future updates.",
        HorizontalAlignment = HorizontalAlignment.Center,
        VerticalAlignment = VerticalAlignment.Center,
        TextWrapping = TextWrapping.Wrap,
        FontSize = 18
      };
    }
  }
}
