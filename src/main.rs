use std::fs;
use rss::Channel;
use chrono::prelude::*;
use webbrowser;
use xdg;
use rayon::prelude::*;

fn main() {
    let xdg_dirs = xdg::BaseDirectories::with_prefix("feed-browser").unwrap();
    let config_file = xdg_dirs.find_config_file("last-read.txt")
        .expect("couldn't find file last-read.txt");
    let feed_file = xdg_dirs.find_config_file("feeds.txt")
        .expect("couldn't find file feeds.txt");
    let last_read_string = fs::read_to_string(config_file.clone())
        .expect("something went wrong reading the file last-read.txt");
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

    match fs::write(config_file, Utc::now().to_rfc3339()) {
        Err(_) => println!("failed to write current datetime back to config file"),
        _ => {},
    }
}
