using System.Data;
using System.IO;
using Parquet;

namespace Dungeon_Crawler_World.AI
{
  public static class ParquetDataLoader
  {
    private static readonly string DataFolder = Path.Combine(path1: AppDomain.CurrentDomain.BaseDirectory, path2: "Data");

    /// <summary>
    /// Loads a Parquet file from the Data folder and returns its contents as a DataTable.
    /// </summary>
    /// <param name="fileName">The name of the Parquet file (e.g., "ai_data.parquet").</param>
    /// <returns>DataTable with the file's contents, or null if not found.</returns>
    public static DataTable LoadParquetFile(string fileName)
    {
      string filePath = Path.Combine(path1: DataFolder, path2: fileName);

      using (Stream fileStream = File.OpenRead(path: filePath))
      {
        using (var parquetReader = ParquetReader.Open(fileStream))
        {
          DataTable table = new DataTable();
          var dataFields = parquetReader.Schema.GetDataFields();

          // Add columns
          foreach (var field in dataFields)
          {
            table.Columns.Add(field.Name, field.ClrNullableIfHasNullsType);
          }

          // Read rows
          for (int i = 0; i < parquetReader.RowGroupCount; i++)
          {
            var columns = parquetReader.ReadEntireRowGroup(i);
            int rowCount = columns[0].Data.Length;
            for (int row = 0; row < rowCount; row++)
            {
              object?[] values = new object?[columns.Length];
              for (int col = 0; col < columns.Length; col++)
              {
                values[col] = columns[col].Data.GetValue(row);
              }
              table.Rows.Add(values);
            }
          }
          return table;
        }
      }
    }
  }
}
