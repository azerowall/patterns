import re
from datetime import datetime, timedelta
import xml.etree.ElementTree as XMLTree
from bs4 import BeautifulSoup
from html.parser import HTMLParser
import requests

from app.models import Channel, Program

class BaseEpgSource:

    def __init__(self):
        self.cache = None

    def get_full(self):
        if self.cache is None:
            self.cache = self._get()
        return self.cache
        
    def get_between_timespan(self, start, end):
        channels = self.get_full()
        result = []
        for channel in channels:
            programs = [p for p in channel.programs if start <= p.time_start and p.time_end <= end]
            result.append(Channel(channel.name, programs))
        return
        
    def get_channel_by_name(self, name):
        for channel in self.get_full():
            if channel.name == name:
                return channel

    def _get(self):
        raise NotImplementedError
    
    
class XmlEpgSource(BaseEpgSource):
    TIME_TEMPLATE = '%Y%m%d%H%M%S %z'
    
    def __init__(self, path):
        super().__init__()
        self._path = path

    def _get(self):
        tree = XMLTree.parse(self._path)
        root = tree.getroot()
        channels = dict()
        
        for e in root:
            if e.tag == 'channel':
                id = int(e.attrib['id'])
                name = next(iter(e)).text
                channels[id] = Channel(name, list())
            elif e.tag == 'programme':
                channel_id = int(e.attrib['channel'])
                name = next(iter(e)).text
                time_start = datetime.strptime(e.attrib['start'], XmlEpgSource.TIME_TEMPLATE)
                time_end = datetime.strptime(e.attrib['stop'], XmlEpgSource.TIME_TEMPLATE)
                channels[channel_id].programs.append(Program(name, time_start, time_end))
                
        return list(channels.values())
                
            
class MailEpgSource(BaseEpgSource):
    URL = 'https://tv.mail.ru/'
    
    def _get(self):
        main_page = requests.get(MailEpgSource.URL + '/orenburg').text
        main_page = BeautifulSoup(main_page, 'lxml')
        links = []
        channels = []
        for e in main_page.select('a.p-channels__item__info__title__link'):
            links.append(e.attrs['href'])
            name = next(iter(e.children))
            channels.append(Channel(name, list()))
        del main_page
        
        for link, channel in zip(links, channels):
            page = requests.get(MailEpgSource.URL + link).text
            page = BeautifulSoup(page, 'lxml')
            for e in page.select('div.p-programms__item'):
                time_start = datetime.strptime(e.attrs['data-start'], '%H:%M')
                time_start = datetime.now().replace(hour=time_start.hour, minute=time_start.minute, second=0, microsecond=0)
                
                name = next(iter(e.select('span.p-programms__item__name-link'))).text
                channel.programs.append(Program(name, time_start, None))
            
            for i, program in enumerate(channel.programs):
                if i + 1 < len(channel.programs):
                    program.time_end = channel.programs[i + 1].time_start
                else:
                    program.time_end = program.time_start + timedelta(hours=1)
                
        return channels

