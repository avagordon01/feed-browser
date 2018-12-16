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
    let mut last_read_string = fs::read_to_string(config_file)
        .expect("something went wrong reading the file config.toml")
        .parse::<toml::value::Datetime>().unwrap()
        .to_string();
    last_read_string = "2018-12-13T23:01:10-08:00".to_string();
    let last_read = DateTime::parse_from_rfc3339(&last_read_string).unwrap();
    fs::read_to_string(feed_file)
        .expect("something went wrong reading the file feeds.txt")
        .lines()
        .for_each(|url| for item in Channel::from_url(url).unwrap().into_items() {
            match DateTime::parse_from_rfc2822(item.pub_date().unwrap()) {
                ParseError => println!("good");
                DateTime<FixedOffset> => println!("good");
                let updated = item_date > last_read;
                if updated {
                    webbrowser::open(item.link().unwrap()).unwrap();
                }
            }
        });
}
