using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace command.TVRemotes
{
    class SonyKeyMapper : IKeyMapper
    {
        public int Map(int nc)
        {
            if (0 <= nc && nc <= 14) return nc;

            throw new KeyMapException(nc);
        }
    }
}
