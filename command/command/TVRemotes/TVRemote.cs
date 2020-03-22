using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using System.Windows;
using System.Windows.Input;

namespace command.TVRemotes
{
    public enum TVRemoteKeyType
    {
        Number, Up = 10, Down, Left, Right, Ok, Back
    }

    public class TVRemoteKey
    {
        public TVRemoteKeyType Type { get; }
        public int Code { get; }
        public int NativeCode { get; }

        public TVRemoteKey(TVRemoteKeyType type, int code, int nativeCode)
        {
            Type = type;
            Code = code;
            NativeCode = nativeCode;
        }
    }

    public class TVRemote
    {
        const int TOTAL_KEYS = 16;
        ICommand[] slots = new ICommand[TOTAL_KEYS];
        IKeyMapper keyMapper = new DummyKeyMapper();
        LogFunc log;

        public TVRemoteKey ClickedKey { get; private set; }

        public void SetKeyMapper(IKeyMapper mapper)
        {
            keyMapper = mapper;
        }

        public delegate void LogFunc(string message);
        public void SetLoggerFunc(LogFunc logFunc)
        {
            log = logFunc;
        }

        public void SetCommand(TVRemoteKeyType type, ICommand command)
        {
            if (type == TVRemoteKeyType.Number)
            {
                for (int i = 0; i < 10; i++)
                    slots[i] = command;
            }
            else
                slots[(int)type] = command;
        }

        public void Reset()
        {
            for (int i = 0; i < slots.Length; i++)
                slots[i] = null;
        }

        public void Click(int nativeCode)
        {
            int code = keyMapper.Map(nativeCode);
            var key = new TVRemoteKey(KeyTypeFromCode(code), code, nativeCode);
            ClickedKey = key;

            log?.Invoke($"Clicked {key.NativeCode} => {key.Type} {key.Code}");
            slots[code]?.Execute(key);
        }

        private TVRemoteKeyType KeyTypeFromCode(int code)
        {
            if (code < 10) return TVRemoteKeyType.Number;
            return (TVRemoteKeyType)code;
        }
        
    }
}
