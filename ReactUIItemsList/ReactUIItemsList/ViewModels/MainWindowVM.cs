using System;
using System.Collections.Generic;
using System.Collections.ObjectModel;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using ReactiveUI;
using ReactiveUI.Fody.Helpers;
using DynamicData;
using DynamicData.Binding;
using ReactUIItemsList.Models;
using System.Collections.Specialized;
using System.Windows.Input;

namespace ReactUIItemsList.ViewModels
{
    class MainWindowVM : ReactiveObject
    {
        public ObservableCollection<Product> Products { get; private set; }

        [Reactive]
        public decimal TotalPrice { get; set; }
        [Reactive]
        public Product SelectedProduct { get; set; }

        public ICommand AddProductCommand { get; private set; }
        public ICommand RemoveProductCommand { get; private set; }

        public MainWindowVM()
        {
            Products = new ObservableCollection<Product>();
            Products.CollectionChanged += (sender, args) => RecalcTotalPrice();
            AddProductCommand = new Commands.DelegateCommand(AddProduct);
            RemoveProductCommand = new Commands.DelegateCommand(RemoveProduct);
        }


        private void RecalcTotalPrice()
        {
            TotalPrice = Products.Sum(p => p.Count * p.Price);
        }

        private void AddProduct(object o)
        {
            var product = new Product(string.Empty, 0, 0);
            product.PropertyChanged += (sender, args) =>
            {
                if (args.PropertyName == "Count" || args.PropertyName == "Price")
                    RecalcTotalPrice();
            };
            Products.Add(product);
        }

        private void RemoveProduct(object o)
        {
            Products.Remove(SelectedProduct);
        }
    }
}
