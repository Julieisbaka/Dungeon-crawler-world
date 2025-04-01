using System;
using System.IO;
using System.Linq;
using System.Text;

class IndexGenerator
{
    static void Main()
    {
        string directoryPath = "Items/Anarchist_cookbook/Book";
        string outputPath = Path.Combine(directoryPath, "Index.md");

        var files = Directory.GetFiles(directoryPath, "*.md")
                             .Select(Path.GetFileName)
                             .OrderBy(f => f)
                             .ToList();

        var sb = new StringBuilder();
        sb.AppendLine("# Index");
        sb.AppendLine();

        for (int i = 0; i < files.Count; i++)
        {
            sb.AppendLine($"{i + 1}. [{Path.GetFileNameWithoutExtension(files[i])}]({files[i]})");
        }

        File.WriteAllText(outputPath, sb.ToString());
    }
}
