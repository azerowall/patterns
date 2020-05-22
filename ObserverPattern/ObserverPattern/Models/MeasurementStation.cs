using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace ObserverPattern.Models
{
    class MeasurementStation
    {
        WeatherData weatherData;
        Random random;
        public MeasurementStation(WeatherData weatherData)
        {
            this.weatherData = weatherData;
            random = new Random();
        }

        public async void Run()
        {
            while (true)
            {
                UpdateMeasurement();
                await Task.Delay(1000);
            }
        }

        private void UpdateMeasurement()
        {
            weatherData.Temperature = random.NextDouble() * 70 - 35;
            weatherData.Humidity = random.NextDouble();
            weatherData.Pressure = random.NextDouble();
            weatherData.Oxygen = random.NextDouble();
            weatherData.Rainfall = random.NextDouble();
            weatherData.Fog = random.Next() % 2 == 1;
            weatherData.Notify(weatherData);
        }
    }
}
