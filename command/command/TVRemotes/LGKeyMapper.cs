using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace command.TVRemotes
{
    class LGKeyMapper : IKeyMapper
    {
        public int Map(int nc)
        {
            if (65 <= nc && nc <= 74)
                return nc - 65;

            switch (nc)
            {
                case 1: return (int)TVRemoteKeyType.Up;
                case 2: return (int)TVRemoteKeyType.Down;
                case 3: return (int)TVRemoteKeyType.Left;
                case 4: return (int)TVRemoteKeyType.Right;
                case 55: return (int)TVRemoteKeyType.Ok;
                case 57: return (int)TVRemoteKeyType.Back;
            }

            throw new KeyMapException(nc);
        }
    }
}
