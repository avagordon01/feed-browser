use reqwest;
use atom_syndication::Feed;
use url::Url;

fn main() {
    let url = "https://jvns.ca/atom.xml";
    let maybe_feed = reqwest::get(url).unwrap()
        .text().unwrap()
        .parse::<Feed>();
    let _entries = match maybe_feed {
        Ok(feed) => {
            let base_url = Url::parse(url).expect("couldn't parse feed url (after having loaded it, wtf)");
            for entry in feed.entries() {
                for link in entry.links() {
                    let new_href = 
                        base_url.join(link.clone().href()).expect("couldn't join entry url with feed url").into_string();
                    link.set_href(new_href)
                }
            }
            feed.entries().to_vec()
        }
        _ => {
            println!("couldn't parse document at url {} as RSS or ATOM!", url);
            vec!{}
        },
    };
}
