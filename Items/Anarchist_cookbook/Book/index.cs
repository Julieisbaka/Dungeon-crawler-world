class IndexGenerator
{
  public static void GenerateIndex()
  {
    const string directoryPath = "Items/Anarchist_cookbook/Book";
    string outputPath = Path.Combine(path1: directoryPath, path2: "index.md");

    List<string?> filesNullable = Directory.GetFiles(path: directoryPath, searchPattern: "*.md")
                         .Select(selector: Path.GetFileName)
                         .OrderBy(keySelector: f => f)
                         .ToList();
    List<string> files = filesNullable.Where(predicate: f => f != null).Cast<string>().ToList();

    StringBuilder sb = new StringBuilder();
    sb.AppendLine(value: "# Index");
    sb.AppendLine();

    for (int i = 0; i < files.Count; i++)
    {
      sb.AppendLine(value: $"{i + 1}. [{Path.GetFileNameWithoutExtension(path: files[index: i])}]({files[index: i]})");
    }

    File.WriteAllText(path: outputPath, contents: sb.ToString());
  }
}
