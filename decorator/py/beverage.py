from abc import ABCMeta, abstractmethod

class Beverage:
    __metaclass__ = ABCMeta
    
    @abstractmethod
    def get_description(self):
        pass

    @abstractmethod
    def cost(self):
        pass

