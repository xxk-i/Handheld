from enum import Enum
from gamefaqs import Gamefaqs

class AvailableProviders(Enum):
    GAMEFAQS = 1
    IGN = 2
    GAMEPRESSURE = 3

class Provider:
    def __init__(self, provider: AvailableProviders):
        self.provider = self.set_provider(provider)
        
    def set_provider(self, provider: AvailableProviders):
        if provider == AvailableProviders.GAMEFAQS:
            return Gamefaqs()
        
        else:
            raise NotImplementedError("Provider is not yet supported")