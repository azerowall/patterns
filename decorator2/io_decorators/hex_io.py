from io_decorator import ReaderDecorator, WriterDecorator
import binascii 

class HexDecoderReader(ReaderDecorator):
    def __init__(self, reader):
        super().__init__(reader)

    def read(self, size: int) -> bytes:
        size *= 2
        data = self.reader.read(size)
        if data is None:
            return None
        return binascii.unhexlify(data)

class HexEncoderWriter(WriterDecorator):
    def __init__(self, writer):
        super().__init__(writer)

    def write(self, data: bytes) -> int:
        data = binascii.hexlify(data)
        return self.writer.write(data) // 2