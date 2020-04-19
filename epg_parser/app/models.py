from dataclasses import dataclass
from datetime import datetime


@dataclass
class Channel:
    name: str
    programs: list
    
    
@dataclass
class Program:
    name: str
    time_start: datetime
    time_end: datetime
    