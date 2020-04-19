


class HTMLTableGenerator:
    def __init__(self):
        self.title = None
        self._html = []
        
    def feed(self, rows):
        for row in rows:
            self._html.append('<td>' + '</td><td>'.join(map(str, row)) + '</td>')
            
    def html(self):
        body = '<tr>' + '</tr><tr>'.join(self._html) + '</tr>'
        if self.title:
            body = f'<tr><th>{self.title}</th></tr>' + body
        return f'<table border="1">{body}</table>'