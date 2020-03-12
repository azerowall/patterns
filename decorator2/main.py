from io_components.mem_io import MemoryReader, MemoryWriter
from io_decorators.buffered_io import BufferedReader
from io_decorators.hex_io import HexEncoderWriter


def demo1():
    reader = open('/etc/passwd', 'rb')
    reader = BufferedReader(reader, 100)

    data = bytes()
    while True:
        d = reader.read(100)
        if d is None:
            break
        data += d

    print(data.decode('utf-8'))

def demo2():
    writer = open('out.txt', 'wb')
    writer = HexEncoderWriter(writer)
    writer.write(b'Hello, world!')
    writer.close()

def main():
    demo2()

if __name__ == '__main__':
    main()