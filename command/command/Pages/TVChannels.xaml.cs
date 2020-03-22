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
using command.TVRemotes;

namespace command.Pages
{
    /// <summary>
    /// Логика взаимодействия для TVChannels.xaml
    /// </summary>
    public partial class TVChannels : Page
    {
        INavigatorService navigator;

        public IEnumerable<Models.TVChannel> Channels => Models.TVChannels.Instance().Channels;
        public Models.TVChannel SelectedChannel { get; set; }
        public int SelectedIndex { get; set; }

        Commands.DelegateCommand FocusUpCommand;
        Commands.DelegateCommand FocusDownCommand;
        Commands.DelegateCommand PlayCommand;
        Commands.DelegateCommand BackCommand;

        public TVChannels(INavigatorService navigatorService, TVRemote tvRemote)
        {
            InitializeComponent();
            DataContext = this;
            navigator = navigatorService;

            FocusUpCommand = new Commands.DelegateCommand(FocusUp);
            FocusDownCommand = new Commands.DelegateCommand(FocusDown);
            PlayCommand = new Commands.DelegateCommand(Play);
            BackCommand = new Commands.DelegateCommand(GoBack);

            lbChannels.SelectedIndex = 0;
            TVRemoteCommandsInit(tvRemote);
        }

        void TVRemoteCommandsInit(TVRemote tvRemote)
        {
            tvRemote.SetCommand(TVRemoteKeyType.Up, FocusUpCommand);
            tvRemote.SetCommand(TVRemoteKeyType.Down, FocusDownCommand);
            tvRemote.SetCommand(TVRemoteKeyType.Ok, PlayCommand);
            tvRemote.SetCommand(TVRemoteKeyType.Back, BackCommand);
        }

        void FocusUp(object o)
        {
            lbChannels.SelectedIndex = (lbChannels.SelectedIndex - 1 + lbChannels.Items.Count) % lbChannels.Items.Count;
        }
        void FocusDown(object o)
        {
            lbChannels.SelectedIndex = (lbChannels.SelectedIndex + 1) % lbChannels.Items.Count;
        }
        void GoBack(object o)
        {
            navigator.Navigate(NavigatorPage.Login);
        }
        void Play(object o)
        {
            navigator.Navigate(NavigatorPage.Player, SelectedChannel);
        }
    }

}
