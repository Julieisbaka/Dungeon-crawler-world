using System.Windows;

namespace Dungeon_Crawler_World.UI
{
    public partial class SettingsPage : Window, IDisposable
    {
        private bool _disposed = false; // To detect redundant calls

        // Implement IDisposable
        public void Dispose()
        {
            Dispose(true);
            GC.SuppressFinalize(this);
        }

        // Protected implementation of Dispose pattern
        protected virtual void Dispose(bool disposing)
        {
            if (!_disposed)
            {
                if (disposing)
                {
                    // Dispose managed resources here, if any.
                }

                // Unmanaged resources are not used. If added in the future, free them here.

                _disposed = true;
            }
        }
    }
}