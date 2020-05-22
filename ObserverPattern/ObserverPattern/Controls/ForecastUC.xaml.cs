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
    /// Логика взаимодействия для ForecastUC.xaml
    /// </summary>
    public partial class ForecastUC : UserControl, IObserver<WeatherData>
    {
        NeuralNetworkPredictor neuralNetworkPredictor;
        public ForecastUC()
        {
            InitializeComponent();
            neuralNetworkPredictor = new NeuralNetworkPredictor();
        }

        public void OnNext(WeatherData weatherData)
        {
            var sb = new StringBuilder();
            sb.AppendLine($"Температура: {DoTheGrandPredict(weatherData.Temperature)}");
            sb.AppendLine($"Влажность: {DoTheGrandPredict(weatherData.Humidity)}");
            sb.AppendLine($"Давление: {DoTheGrandPredict(weatherData.Pressure)}");
            sb.AppendLine($"Процент кислорода: {DoTheGrandPredict(weatherData.Oxygen)}");
            sb.AppendLine($"Осадки: {DoTheGrandPredict(weatherData.Rainfall)}");
            sb.AppendLine($"Туман: {DoTheGrandPredict(weatherData.Fog ? 1 : 0)}");
            tbForecast.Text = sb.ToString();
        }
        public void OnError(Exception e) { }

        public void OnCompleted() { }

        private double DoTheGrandPredict(double value)
        {
            return neuralNetworkPredictor.Predict(value);
        }
    }



















    class NeuralNetworkPredictor
    {
        public double Predict(double value) => LyubaTheGadalka.Predict(value);
    }










    static class LyubaTheGadalka
    {
        static Random random = new Random();

        public static double Predict(double value) => value + random.NextDouble();
    }
}
