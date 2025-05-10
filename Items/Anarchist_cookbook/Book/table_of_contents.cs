class TableOfContentsGenerator
{
  public static void GenerateTableOfContents()
  {
    string directoryPath = "Items/Anarchist_cookbook/Book";
    string outputPath = Path.Combine(path1: directoryPath, path2: "Table_of_contents.md");

    List<string?>? files = Directory.GetFiles(path: directoryPath, searchPattern: "*.md")
                         .Select(selector: Path.GetFileName)
                         .OrderBy(keySelector: f => f)
                         .ToList();

    StringBuilder? sb = new StringBuilder();
    sb.AppendLine(value: "# Table of contents");
    sb.AppendLine();

    for (int i = 0; i < files.Count; i++)
    {
      sb.AppendLine(handler: $"{i + 1}. [{Path.GetFileNameWithoutExtension(path: files[index: i])}]({files[index: i]})");
    }

    File.WriteAllText(path: outputPath, contents: sb.ToString());
  }
}
