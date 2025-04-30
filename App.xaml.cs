using System.Windows;

namespace Dungeon_Crawler_World
{
  public partial class App : Application
  {
    protected override void OnStartup(StartupEventArgs e)
    {
      base.OnStartup(e: e);

      // Generate index and table of contents at startup
      IndexGenerator.GenerateIndex();
      TableOfContentsGenerator.GenerateTableOfContents();
    }
  }
}
