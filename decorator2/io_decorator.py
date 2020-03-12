from io_interface import Reader, Writer

class ReaderDecorator(Reader):
    def __init__(self, reader):
        self.reader = reader

class WriterDecorator(Writer):
    def __init__(self, writer):
        self.writer = writer