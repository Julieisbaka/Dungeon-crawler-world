using System.IO;
using System.Text;
using System.Collections.Generic;
using System.Linq;

public class TableOfContentsGenerator
{
  public static void GenerateTableOfContents()
  {
    string directoryPath = "Items/Anarchist_cookbook/Book";
    string outputPath = Path.Combine(directoryPath, "Table_of_contents.md");

    List<string> files = Directory.GetFiles(directoryPath, "*.md")
                         .Select(Path.GetFileName)
                         .OrderBy(f => f)
                         .ToList();

    StringBuilder sb = new StringBuilder();
    sb.AppendLine("# Table of contents");
    sb.AppendLine();

    for (int i = 0; i < files.Count; i++)
    {
      sb.AppendLine($"{i + 1}. [{Path.GetFileNameWithoutExtension(files[i])}]({files[i]})");
    }

    File.WriteAllText(outputPath, sb.ToString());
  }
}
