using System.ComponentModel;

namespace Dungeon_Crawler_World.UI
{
    public class Settings : INotifyPropertyChanged
    {
        private float reverbLevel = 0.5f;
        private float decayTime = 1.0f;
        private float roomSize = 0.5f;

        public float ReverbLevel
        {
            get => reverbLevel;
            set
            {
                reverbLevel = value;
                OnPropertyChanged(nameof(ReverbLevel));
            }
        }

        public float DecayTime
        {
            get => decayTime;
            set
            {
                decayTime = value;
                OnPropertyChanged(nameof(DecayTime));
            }
        }

        public float RoomSize
        {
            get => roomSize;
            set
            {
                roomSize = value;
                OnPropertyChanged(nameof(RoomSize));
            }
        }

        public event PropertyChangedEventHandler? PropertyChanged;

        protected virtual void OnPropertyChanged(string propertyName)
        {
            PropertyChanged?.Invoke(this, new PropertyChangedEventArgs(propertyName));
        }
    }
}