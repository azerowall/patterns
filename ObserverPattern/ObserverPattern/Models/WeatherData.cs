using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace ObserverPattern.Models
{
    public class WeatherData : Observable<WeatherData>
    {
        public double Temperature { get; set; }
        public double Humidity { get; set; }
        public double Pressure { get; set; }
        public double Oxygen { get; set; }
        public double Rainfall { get; set; }
        public bool Fog { get; set; }
    }
}
