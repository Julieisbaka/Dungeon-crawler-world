using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Text.Json;
using System.Threading.Tasks;
using Microsoft.ML;
using Microsoft.ML.Data;

namespace Dungeon_Crawler_World.AI.Training
{
    public class TrainingData
    {
        private const string DataFilePath = "AI/Data/data.parquet";
        private const string EmotionFilePath = "AI/Data/Emotion.parquet";
        private const string ModelFilePath = "AI/Models/ai_model.zip";

        private MLContext mlContext;
        private ITransformer trainedModel;

        public TrainingData()
        {
            mlContext = new MLContext();
        }

        public void TrainModel()
        {
            IDataView dataView = LoadData(DataFilePath);
            IDataView emotionDataView = LoadData(EmotionFilePath);

            var dataProcessPipeline = mlContext.Transforms.Concatenate("Features", "PlayerActions", "EnvironmentData", "TimeData", "CharacterStats", "EmotionalResponses")
                .Append(mlContext.Transforms.NormalizeMinMax("Features"));

            var trainer = mlContext.Regression.Trainers.Sdca(labelColumnName: "Label", featureColumnName: "Features");
            var trainingPipeline = dataProcessPipeline.Append(trainer);

            trainedModel = trainingPipeline.Fit(dataView);

            SaveModel(trainedModel, dataView.Schema);
        }

        private IDataView LoadData(string filePath)
        {
            return mlContext.Data.LoadFromTextFile<ModelInput>(filePath, hasHeader: true, separatorChar: ',');
        }

        private void SaveModel(ITransformer model, DataViewSchema schema)
        {
            mlContext.Model.Save(model, schema, ModelFilePath);
        }

        public void UpdateKnowledgeBase()
        {
            // Logic to update the AI's knowledge base with new data collected during gameplay
            // This can include appending new data to the existing data files and retraining the model
        }

        public class ModelInput
        {
            [LoadColumn(0)]
            public float PlayerActions { get; set; }

            [LoadColumn(1)]
            public float EnvironmentData { get; set; }

            [LoadColumn(2)]
            public float TimeData { get; set; }

            [LoadColumn(3)]
            public float CharacterStats { get; set; }

            [LoadColumn(4)]
            public float EmotionalResponses { get; set; }

            [LoadColumn(5)]
            public float Label { get; set; }
        }
    }
}
