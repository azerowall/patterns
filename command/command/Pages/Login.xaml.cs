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
    /// Логика взаимодействия для Login.xaml
    /// </summary>
    public partial class Login : Page
    {
        INavigatorService navigator;
        int iFocusedControl = 0;
        UIElement[] controls;


        Commands.DelegateCommand FocusUpCommand;
        Commands.DelegateCommand FocusDownCommand;
        Commands.DelegateCommand InputCommand;
        Commands.DelegateCommand LoginCommand;

        public Login(INavigatorService navigatorService, TVRemote tvRemote)
        {
            navigator = navigatorService;
            InitializeComponent();

            FocusUpCommand = new Commands.DelegateCommand(FocusUp);
            FocusDownCommand = new Commands.DelegateCommand(FocusDown);
            InputCommand = new Commands.DelegateCommand(Input);
            LoginCommand = new Commands.DelegateCommand(DoLogin);

            controls = new[] { tbName, tbPassword, (Control)btnLogin };
            controls[iFocusedControl].Focus();

            TVRemoteCommandsInit(tvRemote);
        }

        void TVRemoteCommandsInit(TVRemote tvRemote)
        {
            tvRemote.SetCommand(TVRemoteKeyType.Up, FocusUpCommand);
            tvRemote.SetCommand(TVRemoteKeyType.Down, FocusDownCommand);
            tvRemote.SetCommand(TVRemoteKeyType.Number, InputCommand);
            tvRemote.SetCommand(TVRemoteKeyType.Ok, LoginCommand);
        }


        #region Commands

        void FocusUp(object o)
        {
            iFocusedControl = (iFocusedControl - 1 + controls.Length) % controls.Length;
            controls[iFocusedControl].Focus();
        }
        void FocusDown(object o)
        {
            iFocusedControl = (iFocusedControl + 1) % controls.Length;
            controls[iFocusedControl].Focus();
        }
        void DoLogin(object o)
        {
            if (string.IsNullOrWhiteSpace(tbName.Text) ||
                string.IsNullOrWhiteSpace(tbPassword.Text))
                return;

            navigator.Navigate(NavigatorPage.Channels);
        }

        void Input(object o)
        {
            var key = o as TVRemoteKey;
            var tb = controls[iFocusedControl] as TextBox;

            if (tb != null)
            {
                tb.Text += key.Code.ToString();
            }
            
        }

        #endregion

    }
}
