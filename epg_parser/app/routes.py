from flask import render_template

from app import app, epg_src
from app.tablegen import HTMLTableGenerator
from app.adapters import ChannelToTableAdapter


@app.route('/')
@app.route('/index')
def index():
    channels = epg_src.get_full()
    channels = (c.name for c in channels)
    return render_template('index.html', channels=channels)
    
    
@app.route('/channel/<name>')
def channel(name):
    channel = epg_src.get_channel_by_name(name)
    if channel:
        gen = HTMLTableGenerator()
        gen.title = channel.name
        gen.feed(ChannelToTableAdapter(channel))
        content = gen.html()
    else:
        content = 'Not Found'
    return render_template('channel.html', content=content)
    
