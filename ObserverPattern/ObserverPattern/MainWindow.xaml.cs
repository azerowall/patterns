using System;
using System.Collections.Generic;
using System.Collections.ObjectModel;
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

namespace ObserverPattern
{
    /// <summary>
    /// Логика взаимодействия для MainWindow.xaml
    /// </summary>
    public partial class MainWindow : Window
    {
        public MainWindow()
        {
            InitializeComponent();

            var weatherData = new WeatherData();

            weatherData.Subscribe(ucCurrentConditions);
            weatherData.Subscribe(ucStatistics);
            weatherData.Subscribe(ucForecast);

            var measurementStation = new MeasurementStation(weatherData);
            measurementStation.Run();
        }
    }

    
}
