use std::fs;
use rss::Channel;
use chrono::prelude::*;
use webbrowser;
use xdg;
use toml;

fn main() {
    let xdg_dirs = xdg::BaseDirectories::with_prefix("feed-browser").unwrap();
    let config_file = xdg_dirs.find_config_file("config.toml")
        .expect("couldn't find file config.toml");
    let feed_file = xdg_dirs.find_config_file("feeds.txt")
        .expect("couldn't find file feeds.txt");
    let mut first = true;
    fs::read_to_string(feed_file)
        .expect("something went wrong reading the file feeds.txt")
        .lines()
        .for_each(|url| for item in Channel::from_url(url).unwrap().into_items() {
            //if DateTime::parse_from_rfc2822(item.pub_date().unwrap()).unwrap() > last_read_date {
            if first {
                webbrowser::open(item.link().unwrap()).unwrap();
                first = false;
            }
        });
}
