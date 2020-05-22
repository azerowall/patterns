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
    /// Логика взаимодействия для StatisticsUC.xaml
    /// </summary>
    public partial class StatisticsUC : UserControl, IObserver<WeatherData>
    {
        ParameterStatistic Temperature = new ParameterStatistic();
        ParameterStatistic Humidity = new ParameterStatistic();
        ParameterStatistic Pressure = new ParameterStatistic();
        ParameterStatistic Oxygen = new ParameterStatistic();
        ParameterStatistic Rainfall = new ParameterStatistic();
        ParameterStatistic Fog = new ParameterStatistic();

        public StatisticsUC()
        {
            InitializeComponent();
        }

        public void OnNext(WeatherData weatherData)
        {
            Temperature.Update(weatherData.Temperature);
            Humidity.Update(weatherData.Humidity);
            Pressure.Update(weatherData.Pressure);
            Oxygen.Update(weatherData.Oxygen);
            Rainfall.Update(weatherData.Rainfall);
            Fog.Update(weatherData.Fog ? 1 : 0);

            Display();
        }
        public void OnError(Exception e) { }

        public void OnCompleted() { }

        public void Display()
        {
            var sb = new StringBuilder();
            sb.AppendLine(StatToString("Температура", Temperature));
            sb.AppendLine(StatToString("Влажность", Humidity));
            sb.AppendLine(StatToString("Давление", Pressure));
            sb.AppendLine(StatToString("Процент кислорода", Oxygen));
            sb.AppendLine(StatToString("Осадки", Rainfall));
            sb.AppendLine(StatToString("Туман", Fog));
            tbStats.Text = sb.ToString();
        }

        private string StatToString(string name, ParameterStatistic stat) =>
            $"{name}: min {Math.Round(stat.Minimum, 2)}; mid {Math.Round(stat.Middle, 2)}; max {Math.Round(stat.Maximum, 2)}";
    }

    class ParameterStatistic
    {
        public double Minimum { get; private set; }
        public double Middle { get; private set; }
        public double Maximum { get; private set; }
        public int NumberOfObservations { get; private set; }

        public ParameterStatistic()
        {
            Minimum = Middle = Maximum = 0;
            NumberOfObservations = 0;
        }

        public void Update(double value)
        {
            if (NumberOfObservations == 0)
            {
                Minimum = Middle = Maximum = value;
                NumberOfObservations = 1;
            }
            else
            {
                Minimum = Math.Min(Minimum, value);
                Maximum = Math.Max(Maximum, value);
                Middle = (Middle * NumberOfObservations + value) / (NumberOfObservations + 1);
            }

            NumberOfObservations += 1;
        }
    }
}
