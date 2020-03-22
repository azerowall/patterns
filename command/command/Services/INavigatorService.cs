using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace command.Services
{
    public enum NavigatorPage
    {
        Login, Channels, Player
    }

    public interface INavigatorService
    {
        void Navigate(NavigatorPage page, object data = null);

        object CurrentPageData { get; }
    }
}
