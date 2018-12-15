use std::fs;
use rss::Channel;
use chrono::prelude::*;
use webbrowser;

fn main() {
    let mut first = true;
    fs::read_to_string("/home/patrick/.config/feed-browser/feeds.txt")
        .expect("Something went wrong reading the file")
        .lines()
        .for_each(|url| for item in Channel::from_url(url).unwrap().into_items() {
            //if DateTime::parse_from_rfc2822(item.pub_date().unwrap()).unwrap() > last_read_date {
            if first {
                webbrowser::open(item.link().unwrap()).unwrap();
                first = false;
            }
        });
}
