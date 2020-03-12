from io_interface import Reader, Writer


class MemoryReader(Reader):
    def __init__(self, buffer: memoryview):
        self.buffer = buffer
        self.cur_pos = 0

    def read(self, size: int) -> bytes:
        if self.cur_pos >= len(self.buffer):
            return None
        data = self.buffer[self.cur_pos : self.cur_pos + size]
        self.cur_pos += size
        return data.tobytes()

    def close(self):
        pass

class MemoryWriter(Reader):
    def __init__(self, buffer: memoryview):
        self.buffer = buffer
        self.cur_pos = 0

    def write(self, data: bytes) -> int:
        if self.cur_pos >= len(self.buffer):
            return 0
        self.buffer[self.cur_pos : self.cur_pos + len(data)] = data
        self.cur_pos += len(data)
        return len(data)

    def flush(self):
        pass