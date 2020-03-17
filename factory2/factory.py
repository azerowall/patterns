
class Factory:
    __slots__ = ('creators')

    def __init__(self):
        self.creators = dict()

    def register(self, name, creator):
        self.creators[name] = creator
    
    def get_registered_names(self):
        return self.creators.keys()

    def make(self, name, *args, **kwargs):
        return self.creators[name](*args, **kwargs)