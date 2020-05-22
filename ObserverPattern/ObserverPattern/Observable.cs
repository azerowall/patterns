using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace ObserverPattern
{
    public class Observable<T> : IObservable<T>
    {
        private List<IObserver<T>> observers = new List<IObserver<T>>();
        public IDisposable Subscribe(IObserver<T> observer)
        {
            observers.Add(observer);
            return new Unsubscriber<T>(this, observer);
        }

        public void Unsubscribe(IObserver<T> observer)
        {
            if (observers.Contains(observer))
                observers.Remove(observer);
        }

        public void Notify(T data)
        {
            foreach (var observer in observers)
                observer.OnNext(data);
        }
    }

    public struct Unsubscriber<T> : IDisposable
    {
        private Observable<T> observable;
        private IObserver<T> observer;
        public Unsubscriber(Observable<T> observable, IObserver<T> observer)
        {
            this.observable = observable;
            this.observer = observer;
        }

        public void Dispose()
        {
            observable.Unsubscribe(observer);
        }
    }
}
