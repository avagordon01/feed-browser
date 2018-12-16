use std::fs;
use rss::Channel;
use chrono::prelude::*;
use webbrowser;
use xdg;
use toml;
use rayon::prelude::*;

fn main() {
    let xdg_dirs = xdg::BaseDirectories::with_prefix("feed-browser").unwrap();
    let config_file = xdg_dirs.find_config_file("config.toml")
        .expect("couldn't find file config.toml");
    let feed_file = xdg_dirs.find_config_file("feeds.txt")
        .expect("couldn't find file feeds.txt");
    let last_read_string = fs::read_to_string(config_file)
        .expect("something went wrong reading the file config.toml");
    let last_read = DateTime::parse_from_rfc3339(&last_read_string.trim()).unwrap();
    let file_string = fs::read_to_string(feed_file)
        .expect("something went wrong reading the file feeds.txt");
    let feed_urls = file_string.par_lines();
    let items = feed_urls.flat_map(|url| {
        match Channel::from_url(url) {
            Ok(items) => {items.into_items()},
            _ => vec!{},
        }
    });
    let new_items = items.filter(|item| {
        match item.pub_date() {
            Some(d) => {
                match DateTime::parse_from_rfc2822(d) {
                    Ok(item_date) => item_date > last_read,
                    _ => false,
                }
            }
            _ => false,
        }
    });
    let new_links = new_items.filter_map(|i| {
        match i.link() {
            Some(l) => Some(l.to_owned()),
            _ => None,
        }
    });
    new_links.for_each(|l| {webbrowser::open(&l).unwrap();});
}
