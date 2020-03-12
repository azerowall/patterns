from io_decorator import ReaderDecorator, WriterDecorator


class BufferedReader(ReaderDecorator):
    def __init__(self, reader, buffer_size = 100):
        super().__init__(reader)
        self.buffer = memoryview(bytearray(0))
        self.buffer_size = buffer_size

    def read(self, size: int) -> bytes:
        if len(self.buffer) >= size:
            #print(f'Extract data ({size} bytes) from buffer ({len(self.buffer)} bytes)')
            data = self.buffer[:size]
            self.buffer = self.buffer[size:]
            return data.tobytes()
        else:
            data = self.buffer.tobytes()

            needed_data_size = size - len(data)
            data_from_reader = self.reader.read(needed_data_size + self.buffer_size)

            #print(f'data from buffer size = {len(data)}')
            #print(f'needed_data_size = {needed_data_size}')
            #print(f'data_from_reader size = {len(data_from_reader)}')
            if data_from_reader is not None:
                data = data + data_from_reader[:needed_data_size]
                self.buffer = memoryview(data_from_reader[needed_data_size:])

            if len(data) is 0:
                return None

            #print(f'data = from buffer ({len(self.buffer)} bytes) + from reader ({needed_data_size} bytes)')
            
            #print(f'new buffer = {len(self.buffer)} ({self.buffer_size})')
            return data


