using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace command.Models
{
    public class TVChannel
    {
        public int Id { get; set; }
        public string Name { get; set; }
        public string Program { get; set; }
    }

    public class TVChannels
    {
        public IEnumerable<TVChannel> Channels { get; private set; }

        static TVChannels instance = new TVChannels();
        public static TVChannels Instance() => instance;

        TVChannels()
        {
            Channels = new[]
            {
                new TVChannel { Id = 0, Name = "Первый", Program = "Малышева"},
                new TVChannel { Id = 1, Name = "Первый", Program = "Малышева"},
                new TVChannel { Id = 2, Name = "Первый", Program = "Малышева"},
                new TVChannel { Id = 3, Name = "Первый", Program = "Малышева"},
                new TVChannel { Id = 4, Name = "Первый", Program = "Малышева"},
            };
        }

        public TVChannel GetChannelById(int id)
        {
            return Channels.Where(channel => channel.Id == id).First();
        }
    }
}
