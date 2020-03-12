from abc import ABCMeta, abstractmethod
from collections import Counter

class Beverage:
    __metaclass__ = ABCMeta
    
    def get_description(self):
        counts = Counter(self._get_components())
        return ', '.join(f'{stringify(count, what)}'
            for (what, count) in counts.items())

    
    def _get_components(self):
        return [ self._get_own_description() ]

    @abstractmethod
    def cost(self):
        pass

    @abstractmethod
    def _get_own_description(self):
        pass

def stringify(count, component):
    if count is 1:
        return component
    words = ['One', 'Double', 'Triple', 'Quadruple', 'Quintuple', 'Sextuple', 'Septuple', 'Octuple']
    if count < len(words):
        return f'{words[count - 1]} {component}'
    return f'{count}-tuple {component}'