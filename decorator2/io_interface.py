from abc import ABCMeta, abstractmethod
import typing

class Reader:
    __metaclass__ = ABCMeta

    @abstractmethod
    def read(self, size: int) -> bytes:
        pass

    @abstractmethod
    def close(self):
        pass


class Writer:
    __metaclass__ = ABCMeta

    @abstractmethod
    def write(self, data: bytes) -> int:
        pass

    @abstractmethod
    def flush(self):
        pass

    @abstractmethod
    def close(self):
        pass