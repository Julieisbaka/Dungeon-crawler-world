using System.Windows;
using System.Windows.Navigation;

namespace Dungeon_Crawler_World
{
  public partial class App : Application
  {
    protected override void OnStartup(StartupEventArgs e)
    {
      base.OnStartup(e: e);

      NavigationWindow navigationWindow = new NavigationWindow(); // Unnecessary assignment of a value to 'navigationWindow'IDE0059



      // Generate index and table of contents at startup
      IndexGenerator.GenerateIndex();
      TableOfContentsGenerator.GenerateTableOfContents();
    }
  }
}
