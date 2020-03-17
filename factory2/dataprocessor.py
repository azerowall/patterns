from abc import ABCMeta, abstractmethod

class DataProcessor:
    
    @abstractmethod
    def process(self, data: memoryview) -> memoryview:
        pass

    @abstractmethod
    def flush(self) -> memoryview:
        pass
