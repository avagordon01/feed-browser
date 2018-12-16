use std::fs;
use rss::Channel;
use chrono::prelude::*;
use webbrowser;
use xdg;
use std::iter::FlatMap;

fn main() {
    let xdg_dirs = xdg::BaseDirectories::with_prefix("feed-browser").unwrap();
    let config_dir = xdg_dirs.find_config_file("config.ini").unwrap();
    let feed_files = xdg_dirs.list_config_files("feeds.txt");
    println!("{}", config_dir.to_str().unwrap());
    let mut first = false;
    feed_files.into_iter().flat_map(|filename| fs::read_to_string(filename)
        .expect("Something went wrong reading the file")
        .lines().cloned()
        )
        .for_each(|url| for item in Channel::from_url(url).unwrap().into_items() {
            //if DateTime::parse_from_rfc2822(item.pub_date().unwrap()).unwrap() > last_read_date {
            if first {
                webbrowser::open(item.link().unwrap()).unwrap();
                first = false;
            }
        });
}
