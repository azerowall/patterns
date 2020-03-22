using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace command.TVRemotes
{
    public interface IKeyMapper
    {
        int Map(int nativeCode);
    }

    class KeyMapException : Exception
    {
        public int NativeCode { get; private set; }
        public KeyMapException(int ncode)
        {
            NativeCode = ncode;
        }
    }
}
