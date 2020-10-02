from http.server import HTTPServer, SimpleHTTPRequestHandler
import webbrowser 
import os

# This script can be used to run helixiser locally. 
# call it in the /helixiser directory (repository root)
# The imports should all be included with your python installation,
# but if they are absent you can install them with e.g. pip

server_address = ('0.0.0.0', 8080)    
httpd = HTTPServer(server_address, SimpleHTTPRequestHandler)
os.chdir('../') # one directory up, so we can get the /helixiser/ path right.

print('Opening localhost:8080/helixiser/ in browser...')
webbrowser.open_new('http://localhost:8080/helixiser/') 

httpd.serve_forever()
