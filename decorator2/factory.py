
class Factory:
    __slots__ = ('builders')

    def __init__(self):
        self.builders = dict()

    def register(self, name, builder):
        self.builders[name] = builder

    def make(self, name, *args, **kwargs):
        return self.builders[name](*args, **kwargs)