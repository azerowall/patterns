from flask import request
from datetime import timedelta

from app import app, epg_src



@app.route('/api/channel.get_average_program_length')
def get_average_program_length():
    channel_name = request.args.get('channel')
    channel = epg_src.get_channel_by_name(channel_name)
    if channel is None:
        return 'null'
        
    avg = sum((p.time_end - p.time_start for p in channel.programs), timedelta(0)) / len(channel.programs)
        
    return str(avg)