

class ChannelToTableAdapter:
    def __init__(self, channel):
        self.channel = channel
    
    def __iter__(self):
        program_adapter = lambda p: (p.time_start.strftime('%H:%M'),
                                     p.time_end.strftime('%H:%M'),
                                     p.name)
        return map(program_adapter, self.channel.programs)