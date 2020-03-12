from beverage_decorator import BeverageDecorator
from config import Config

class Milk(BeverageDecorator):
    def __init__(self, beverage):
        super().__init__(beverage)

    def cost(self):
        return self.beverage.cost() + Config['milk.cost']

    def _get_own_description(self):
        return Config['milk.desc']


class Mocha(BeverageDecorator):
    def __init__(self, beverage):
        super().__init__(beverage)
    def cost(self):
        return self.beverage.cost() + Config['mocha.cost']
    def _get_own_description(self):
        return Config['mocha.desc']


class Soy(BeverageDecorator):
    def __init__(self, beverage):
        super().__init__(beverage)
    def cost(self):
        return self.beverage.cost() + Config['soy.cost']
    def _get_own_description(self):
        return Config['soy.desc']


class Whip(BeverageDecorator):
    def __init__(self, beverage):
        super().__init__(beverage)
    def cost(self):
        return self.beverage.cost() + Config['whip.cost']
    def _get_own_description(self):
        return Config['whip.desc']

class Caramel(BeverageDecorator):
    def __init__(self, beverage):
        super().__init__(beverage)
    def cost(self):
        return self.beverage.cost() + Config['caramel.cost']
    def _get_own_description(self):
        return Config['caramel.desc']


class Discont(BeverageDecorator):
    def __init__(self, beverage, deductible_cost):
        super().__init__(beverage)
        self.deductible_cost = deductible_cost
    def cost(self):
        return self.beverage.cost() - self.deductible_cost
    def _get_own_description(self):
        return Config['discont.desc']