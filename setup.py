from setuptools import setup

setup(
    name='feed-browser',
    version='0.1',
    scripts=['feed-browser'],
    install_requires=[
        'opml',
        'feedparser',
    ],
)
