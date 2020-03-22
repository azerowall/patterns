using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using System.Windows;
using System.Windows.Controls;
using System.Windows.Data;
using System.Windows.Documents;
using System.Windows.Input;
using System.Windows.Media;
using System.Windows.Media.Imaging;
using System.Windows.Navigation;
using System.Windows.Shapes;
using command.Services;

namespace command
{
    /// <summary>
    /// Логика взаимодействия для MainWindow.xaml
    /// </summary>
    public partial class MainWindow : Window, INavigatorService
    {
        TVRemotes.TVRemote tvRemote = new TVRemotes.TVRemote();
        public MainWindow()
        {
            InitializeComponent();
            Navigate(NavigatorPage.Login);

            tvRemote.SetLoggerFunc(Log);
        }

        public object CurrentPageData { get; private set; }
        public void Navigate(NavigatorPage page, object data = null)
        {
            CurrentPageData = data;
            tvRemote.Reset();
            switch (page)
            {
                case NavigatorPage.Login:
                    pageFrame.Content = new Pages.Login(this, tvRemote);
                    break;
                case NavigatorPage.Channels:
                    pageFrame.Content = new Pages.TVChannels(this, tvRemote);
                    break;
                case NavigatorPage.Player:
                    pageFrame.Content = new Pages.Player(this, tvRemote);
                    break;
            }
        }

        private void tbTVRemoteCommand_KeyDown(object sender, KeyEventArgs e)
        {
            try
            {
                if (e.Key != Key.Enter) return;
                if (int.TryParse(tbTVRemoteCommand.Text, out int code))
                {
                    tvRemote.Click(code);
                }
                tbTVRemoteCommand.Text = string.Empty;
            }
            catch (TVRemotes.KeyMapException ex)
            {
                Log($"Can't map {ex.NativeCode}");
            }
        }

        private void Log(string message)
        {
            tbLog.Text += message + '\n';
        }

        private void sbTVRemotes_SelectionChanged(object sender, SelectionChangedEventArgs e)
        {
            TVRemotes.IKeyMapper mapper;
            switch (sbTVRemotes.SelectedIndex)
            {
                case 0:
                    mapper = new TVRemotes.SamsungKeyMapper();
                    break;
                case 1:
                    mapper = new TVRemotes.LGKeyMapper();
                    break;
                case 2:
                    mapper = new TVRemotes.SonyKeyMapper();
                    break;
                default:
                    return;

            }
            tvRemote.SetKeyMapper(mapper);
        }
    }
}
