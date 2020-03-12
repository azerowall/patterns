import beverage


class BeverageDecorator(beverage.Beverage):

    def __init__(self, beverage):
        self.beverage = beverage

    def _get_components(self):
        return self.beverage._get_components() + [ self._get_own_description() ]