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
using ObserverPattern.Models;

namespace ObserverPattern.Controls
{
    /// <summary>
    /// Логика взаимодействия для CurrentConditionUC.xaml
    /// </summary>
    public partial class CurrentConditionUC : UserControl, IObserver<Models.WeatherData>
    {
        public CurrentConditionUC()
        {
            InitializeComponent();
        }

        public void OnNext(WeatherData weatherData)
        {
            lblTemperature.Content = weatherData.Temperature.ToString();
            lblHumidity.Content = weatherData.Humidity.ToString();
            lblPressure.Content = weatherData.Pressure.ToString();
            lblOxygen.Content = weatherData.Oxygen.ToString();
            lblRainfall.Content = weatherData.Rainfall.ToString();
            lblFog.Content = weatherData.Fog.ToString();
        }
        public void OnError(Exception e) { }

        public void OnCompleted() { }
    }
}
