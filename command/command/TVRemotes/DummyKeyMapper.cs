using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace command.TVRemotes
{
    class DummyKeyMapper : IKeyMapper
    {
        public int Map(int nc) => nc;
    }
}
