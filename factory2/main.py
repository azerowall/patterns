import sys

from coders import *
from factory import Factory




factory = Factory()

factory.register('b32enc', Base32Encoder)
factory.register('b32dec', Base32Decoder)
factory.register('b64enc', Base64Encoder)
factory.register('b64dec', Base64Decoder)
factory.register('b85enc', Base85Encoder)
factory.register('b85dec', Base85Decoder)
factory.register('a85enc', Ascii85Encoder)
factory.register('a85dec', Ascii85Decoder)
factory.register('hexenc', HexEncoder)
factory.register('hexdec', HexDecoder)


def main():
    if len(sys.argv) < 3:
        print('\n'.join(factory.get_registered_names()))
        return
    else:
        coder = factory.make(sys.argv[1])
        result = coder.process(sys.argv[2].encode('utf-8'))
        result += coder.flush()
        print(result.tobytes())


if __name__ == '__main__':
    main()