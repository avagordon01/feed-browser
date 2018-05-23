#!/usr/bin/python3
import os
import sys
import opml
import feedparser
import concurrent.futures
import time
import configparser
import webbrowser
import glob

config_dir = os.environ.get('XDG_CONFIG_HOME') or os.path.expandvars(os.path.join('$HOME', '.config'))
config_dir = os.path.join(config_dir, 'feed-browser')
config_filename = os.path.join(config_dir, 'config.ini')
config = configparser.ConfigParser()
config.read(config_filename)
try:
    last_read = time.strptime(config['feed-browser']['last-read'])
except KeyError:
    last_read = time.gmtime(0)

opml_filename = glob.glob(os.path.join(config_dir, '*.opml'))[0]
opml_file = opml.parse(opml_filename)

rss_urls = [x.xmlUrl for y in opml_file for x in y]

def load_url(rss_url):
    return feedparser.parse(rss_url)
with concurrent.futures.ThreadPoolExecutor(max_workers = 256) as executor:
    feeds = executor.map(load_url, rss_urls)

entries = []
for feed in feeds:
    entries.extend(feed["items"])

def entry_time(entry):
    if 'updated_parsed' in entry:
        t = entry['updated_parsed']
    elif 'published_parsed' in entry:
        t = entry['published_parsed']
    if t is not None:
        return t
    else:
        return time.gmtime(0)
def new_entry(entry):
    return entry_time(entry) > last_read
entries = sorted(filter(new_entry, entries), key=entry_time, reverse=True)

for x in entries:
    webbrowser.open(x.link)

config['feed-browser'] = {'last-read': time.asctime()}
with open(config_filename, 'w') as config_file:
    config.write(config_file)
