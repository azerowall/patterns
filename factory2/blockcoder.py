from aggregator import Aggregator

class BlockCoder:
    def __init__(self, code_f, needed_size, can_be_flushed):
        self.code_fun = code_f
        self.needed_size = needed_size
        self.can_be_flushed = can_be_flushed
        self.aggregator = Aggregator()

    def process(self, data: memoryview) -> memoryview:
        self.aggregator.push(data)
        result = bytearray()
        while self.aggregator.has_pull_data(self.needed_size):
            d = self.aggregator.pull(self.needed_size)
            result.extend(self.code_fun(d))
        return memoryview(result)

    def flush(self) -> memoryview:
        if self.aggregator.total_data_size() == 0:
            return bytes()
        if self.can_be_flushed:
            result = self.code_fun(self.aggregator.flush())
            return memoryview(result)
        else:
            raise IOError()