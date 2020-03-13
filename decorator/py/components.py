from beverage import Beverage
from config import Config

class DarkRoast(Beverage):
    
    def cost(self):
        return Config['dark_roast.cost']

    def get_description(self):
        return Config['dark_roast.desc']


class HouseBlend(Beverage):
    def cost(self):
        return Config['house_blend.cost']
    def get_description(self):
        return Config['house_blend.desc']

class Espresso(Beverage):
    def cost(self):
        return Config['espresso.cost']
    def get_description(self):
        return Config['espresso.desc']

class Decaf(Beverage):
    def cost(self):
        return Config['decaf.cost']
    def get_description(self):
        return Config['decaf.desc']

class Tea(Beverage):
    def cost(self):
        return Config['tea.cost']
    def get_description(self):
        return Config['tea.desc']