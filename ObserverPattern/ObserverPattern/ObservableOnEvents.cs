using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace ObserverPattern
{
    public class ObservableOnEvents<T>
    {
        private event Action<T> Event;
        public IDisposable Subscribe(IObserver<T> observer)
        {
            Event += observer.OnNext;
            return new UnsubscriberOnEvents<T>(this, observer);
        }

        public void Unsubscribe(IObserver<T> observer)
        {
            Event -= observer.OnNext;
        }

        public void Notify(T data)
        {
            Event?.Invoke(data);
        }
    }

    public struct UnsubscriberOnEvents<T> : IDisposable
    {
        private ObservableOnEvents<T> observable;
        private IObserver<T> observer;
        public UnsubscriberOnEvents(ObservableOnEvents<T> observable, IObserver<T> observer)
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
