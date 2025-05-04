using System;
using System.IO;
using System.Text.Json;
using System.Windows;
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
                string schemaPath = Path.Combine(AppDomain.CurrentDomain.BaseDirectory, "Scheme", "Shader.json");
                if (!File.Exists(schemaPath))
                {
                    error = $"Shader schema not found: {schemaPath}";
                    return false;
                }
                string schemaJson = File.ReadAllText(schemaPath);
                JSchema schema = JSchema.Parse(schemaJson);
                JObject configObj = JObject.FromObject(shaderConfig);
                if (!configObj.IsValid(schema, out IList<string> errors))
                {
                    error = string.Join("\n", errors);
                    return false;
                }
                return true;
            }
            catch (Exception ex)
            {
                error = ex.Message;
                return false;
            }
        }
    }
}
