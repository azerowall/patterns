from beverage_decorator import BeverageDecorator
from config import Config
from collections import Counter

class Milk(BeverageDecorator):
    def __init__(self, beverage):
        super().__init__(beverage)

    def cost(self):
        return self.beverage.cost() + Config['milk.cost']

    def get_description(self):
        return self.beverage.get_description() + ", " + Config['milk.desc']


class Mocha(BeverageDecorator):
    def __init__(self, beverage):
        super().__init__(beverage)
    def cost(self):
        return self.beverage.cost() + Config['mocha.cost']
    def get_description(self):
        return self.beverage.get_description() + ", " + Config['mocha.desc']


class Soy(BeverageDecorator):
    def __init__(self, beverage):
        super().__init__(beverage)
    def cost(self):
        return self.beverage.cost() + Config['soy.cost']
    def get_description(self):
        return self.beverage.get_description() + ", " +  Config['soy.desc']


class Whip(BeverageDecorator):
    def __init__(self, beverage):
        super().__init__(beverage)
    def cost(self):
        return self.beverage.cost() + Config['whip.cost']
    def get_description(self):
        return self.beverage.get_description() + ", " +  Config['whip.desc']

class Caramel(BeverageDecorator):
    def __init__(self, beverage):
        super().__init__(beverage)
    def cost(self):
        return self.beverage.cost() + Config['caramel.cost']
    def get_description(self):
        return self.beverage.get_description() + ", " +  Config['caramel.desc']


class Discont(BeverageDecorator):
    def __init__(self, beverage, deductible_cost):
        super().__init__(beverage)
        self.deductible_cost = deductible_cost
    def cost(self):
        return self.beverage.cost() - self.deductible_cost
    def get_description(self):
        return self.beverage.get_description() + ", " +  Config['discont.desc']
        
        
class BeautifyDescription(BeverageDecorator):
    def __init__(self, beverage):
        super().__init__(beverage)
        
    def get_description(self):
        counts = Counter(self.beverage.get_description().split(", "))
        return ', '.join(self.stringify(count, comp) for comp, count in counts.items())
        
    def stringify(self, count, component):
        if count is 1:
            return component
        words = ['One', 'Double', 'Triple', 'Quadruple', 'Quintuple', 'Sextuple', 'Septuple', 'Octuple']
        if count < len(words):
            return f'{words[count - 1]} {component}'
        return f'{count}-tuple {component}'