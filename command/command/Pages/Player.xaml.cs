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
    /// Логика взаимодействия для Player.xaml
    /// </summary>
    public partial class Player : Page
    {
        INavigatorService navigator;
        public Models.TVChannel Channel { get; set; }

        Commands.DelegateCommand BackCommand;

        public Player(INavigatorService navigatorService, TVRemote tvRemote)
        {
            InitializeComponent();
            DataContext = this;
            navigator = navigatorService;
            Channel = navigator.CurrentPageData as Models.TVChannel;

            BackCommand = new Commands.DelegateCommand(GoBack);

            TVRemoteCommandsInit(tvRemote);
        }

        void TVRemoteCommandsInit(TVRemote tvRemote)
        {
            tvRemote.SetCommand(TVRemoteKeyType.Ok, BackCommand);
            tvRemote.SetCommand(TVRemoteKeyType.Back, BackCommand);
        }

        void GoBack(object o)
        {
            navigator.Navigate(NavigatorPage.Channels);
        }
    }
}
