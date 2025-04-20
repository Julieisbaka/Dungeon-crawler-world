namespace DungeonCrawlerWorld.SoundCommon
{
    public class VoiceChatButton : Form
    {
        private Button voiceChatButton;
        private ContextMenuStrip configMenu;

        public VoiceChatButton()
        {
            // Initialize the button
            voiceChatButton = new Button
            {
                Text = "Voice Chat",
                Width = 120,
                Height = 40,
                Top = 20,
                Left = 20
            };
            voiceChatButton.Click += VoiceChatButton_Click;

            // Initialize the context menu
            configMenu = new ContextMenuStrip();
            configMenu.Items.Add("Volume Settings", null, (s, e) => ShowConfigMessage("Volume Settings"));
            configMenu.Items.Add("Microphone Settings", null, (s, e) => ShowConfigMessage("Microphone Settings"));
            configMenu.Items.Add("Push-to-Talk Settings", null, (s, e) => ShowConfigMessage("Push-to-Talk Settings"));

            // Attach the context menu to the button
            voiceChatButton.ContextMenuStrip = configMenu;

            // Add the button to the form
            Controls.Add(voiceChatButton);

            // Configure the form
            Text = "Voice Chat Configurations";
            Width = 300;
            Height = 150;
            StartPosition = FormStartPosition.CenterScreen;
        }

        private void VoiceChatButton_Click(object sender, EventArgs e)
        {
            MessageBox.Show("This feature is still in progress", "Voice Chat", MessageBoxButtons.OK, MessageBoxIcon.Information);
        }

        private void ShowConfigMessage(string configName)
        {
            MessageBox.Show($"Opening {configName} configuration...", "Configuration", MessageBoxButtons.OK, MessageBoxIcon.Information);
        }

        [STAThread]
        public static void Main()
        {
            Application.EnableVisualStyles();
            Application.SetCompatibleTextRenderingDefault(false);
            Application.Run(new VoiceChatButton());
        }
    }
}