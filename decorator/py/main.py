from decorators import *
from components import *
import random

class Factory:
    def __init__(self):
        self.builders = dict()
    def register(self, name, builder):
        self.builders[name] = builder

    def make(self, name, *args, **kwargs):
        return self.builders[name](*args, **kwargs)
        
    def get_registered(self):
        return self.builders.keys()

        
class Cafe:
    def __init__(self):
        decorators_factory = Factory()
        decorators_factory.register('milk', Milk)
        decorators_factory.register('mocha', Mocha)
        decorators_factory.register('soy', Soy)
        decorators_factory.register('whip', Whip)
        decorators_factory.register('caramel', Caramel)
        
        components_factory = Factory()
        components_factory.register('dark_roast', DarkRoast)
        components_factory.register('house_blend', HouseBlend)
        components_factory.register('espresso', Espresso)
        components_factory.register('decaf', Decaf)
        components_factory.register('tea', Tea)
        
        self.decorators_factory = decorators_factory
        self.components_factory = components_factory
        
    def get_base_components(self):
        return self.components_factory.get_registered()
        
    def get_additives(self):
        return self.decorators_factory.get_registered()
        
    def make_beverage(self, base_component, additives):
        beverage = self.components_factory.make(base_component)
        
        for additive in additives:
            beverage = self.decorators_factory.make(additive, beverage)

        if random.randint(0, 1) is 1:
            beverage = Discont(beverage, 0.12)
            
        return BeautifyDescription(beverage)
        
        




def demo1():
    beverage = HouseBlend()
    beverage = Milk(beverage)
    beverage = Soy(beverage)
    beverage = Milk(beverage)
    beverage = Milk(beverage)
    beverage = Caramel(beverage)
    beverage = Caramel(beverage)
    beverage = Whip(beverage)
    beverage = Milk(beverage)
    beverage = Discont(beverage, 0.12)
    beverage = BeautifyDescription(beverage)

    print('Ваша овсянка, сэр:')
    print('Description:', beverage.get_description())
    print('Cost:', round(beverage.cost(), 5))
    
    
def main():
    cafe = Cafe()
    print('Доступные напитки:', ', '.join(cafe.get_base_components()))
    base = input('Введите напиток: ')
    
    print('Доступные добавки:', ', '.join(cafe.get_additives()))
    additives = input('Введите добавки через запятую: ').split(',')
    additives = [a.strip() for a in additives]
    
    beverage = cafe.make_beverage(base, additives)
    
    print('Ваша овсянка:', beverage.get_description())
    print('С вас:', round(beverage.cost(), 4))


if __name__ == '__main__':
    main()