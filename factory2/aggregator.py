class Aggregator:
    '''
        Гарантирует, что считываемые данные будут определенного размера
    '''
    def __init__(self):
        self.buffers = []
        self.total_size = 0
        self.pos = 0

    def push(self, data: memoryview):
        self.buffers.append(data)
        self.total_size += len(data)

    def has_pull_data(self, size: int):
        return self.total_size >= size

    def total_data_size(self):
        return self.total_size

    def pull(self, size: int) -> memoryview:
        data = memoryview(self.buffers[0])[self.pos : self.pos + size]
        self.pos += len(data)
        if self.pos == len(self.buffers[0]):
            self.pos = 0
            del self.buffers[0]

        if len(data) < size:
            data = bytearray(data)
            while len(data) < size:
                remained_size = size - len(data)
                d = memoryview(self.buffers[0])[:remained_size]
                data.extend(d)
                if len(d) != remained_size:
                    del self.buffers[0]
                elif len(d) < self.buffers[0]:
                    self.pos = len(d)
        
        self.total_size -= size
        return data

    def flush(self):
        pass