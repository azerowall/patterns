using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using ReactiveUI;
using ReactiveUI.Fody.Helpers;

namespace ReactUIItemsList.Models
{
    class Product : ReactiveObject
    {
        [Reactive]
        public string Name { get; set; }

        [Reactive]
        public int Count { get; set; }

        [Reactive]
        public decimal Price { get; set; }

        public Product(string name, int count, decimal price)
        {
            Name = name;
            Count = count;
            Price = price;
        }
    }
}
