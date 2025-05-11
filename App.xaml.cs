using System.Windows;
using System.Windows.Navigation;

namespace Dungeon_Crawler_World
{
  public partial class App : Application
  {
    protected override void OnStartup(StartupEventArgs e)
    {
      base.OnStartup(e: e);

      NavigationWindow navigationWindow = new NavigationWindow();
      try
      {
        // The Main page is in the UI namespace and likely in a UI folder.
        navigationWindow.Source = new Uri(uriString: "UI/Main.xaml", uriKind: UriKind.Relative);
      }
      catch (System.IO.IOException ex)
      {
        // Handle cases where the XAML file might not be found.
        MessageBox.Show(messageBoxText: $"Error loading page: UI/Main.xaml\n{ex.Message}", caption: "Startup Error", button: MessageBoxButton.OK, icon: MessageBoxImage.Error);
        // Optionally, shutdown or load a fallback page.
        // this.Shutdown();
        // return;
      }


      // Generate index and table of contents at startup
      IndexGenerator.GenerateIndex();
      TableOfContentsGenerator.GenerateTableOfContents();
    }
  }
}
