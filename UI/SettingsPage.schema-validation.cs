using System.IO;
using Newtonsoft.Json.Linq;
using Newtonsoft.Json.Schema;

namespace Dungeon_Crawler_World.UI
{
    public partial class SettingsPage
    {
        private static bool ValidateShaderConfig(object shaderConfig, out string error)
        {
            error = string.Empty;
            try
            {
                string schemaPath = Path.Combine(path1: AppDomain.CurrentDomain.BaseDirectory, path2: "Scheme", path3: "Shader.json");
                if (!File.Exists(path: schemaPath))
                {
                    error = $"Shader schema not found: {schemaPath}";
                    return false;
                }
                string schemaJson = File.ReadAllText(path: schemaPath);
                JSchema schema = JSchema.Parse(json: schemaJson);
                JObject configObj = JObject.FromObject(o: shaderConfig);
                if (!configObj.IsValid(schema: schema, errorMessages: out IList<string> errors))
                {
                    error = string.Join(separator: "\n", values: errors);
                    return false;
                }
                return true;
            }
            catch (FileNotFoundException ex)
            {
                error = $"File not found: {ex.Message}";
                return false;
            }
            catch (JsonReaderException ex)
            {
                error = $"JSON parsing error: {ex.Message}";
                return false;
            }
            catch (JSchemaException ex)
            {
                error = $"Schema validation error: {ex.Message}";
                return false;
            }
            catch (Exception ex)
            {
                error = $"Unexpected error: {ex.Message}";
                return false;
            }
        }
    }
}
