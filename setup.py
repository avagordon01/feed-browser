from setuptools import setup

setup(
    name='feed-browser',
    version='0.1.4',
    scripts=['feed-browser/feed-browser'],
    install_requires=[
        'opml',
        'feedparser',
    ],
)
