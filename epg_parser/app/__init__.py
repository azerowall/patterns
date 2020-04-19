from flask import Flask

from app.epg_sources import XmlEpgSource, MailEpgSource


app = Flask(__name__)

#epg_src = XmlEpgSource('app/xmltv.xml')
epg_src = MailEpgSource()

from app import routes, api, models