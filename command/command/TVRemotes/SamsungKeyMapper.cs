using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace command.TVRemotes
{
    class SamsungKeyMapper : IKeyMapper
    {
        public int Map(int nativeCode)
        {
            if (23 <= nativeCode && nativeCode <= 32)
                return nativeCode - 23;

            switch (nativeCode)
            {
                case 56: return (int)TVRemoteKeyType.Up;
                case 58: return (int)TVRemoteKeyType.Down;
                case 57: return (int)TVRemoteKeyType.Left;
                case 55: return (int)TVRemoteKeyType.Right;
                case 123: return (int)TVRemoteKeyType.Ok;
                case 321:
                case 45: return (int)TVRemoteKeyType.Back;
            }

            throw new KeyMapException(nativeCode);
        }
    }
}
