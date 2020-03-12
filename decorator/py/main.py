from decorators import *
from components import *

class Factory:
    def __init__(self):
        self.builders = dict()
    def register(self, name, builder):
        self.builders[name] = builder

    def make(self, name, *args, **kwargs):
        return self.builders[name](*args, **kwargs)





def main():
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

    print('Ваша овсянка, сэр:')
    print('Description:', beverage.get_description())
    print('Cost:', round(beverage.cost(), 5))


if __name__ == '__main__':
    main()