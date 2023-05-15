use crate::loader::scraper::Scraper;
use regex::Regex;

trait UrlParser {
    fn from_url(&self, url: String) -> Option<Scraper>;
}

pub fn from_url(url: String) -> Option<Scraper> {
    let parsers: Vec<Box<dyn UrlParser>> = vec![Box::new(PixivParser)];
    for parser in parsers {
        let scraper = parser.from_url(url.clone());
        if scraper.is_some() {
            return scraper;
        }
    }
    None
}

struct PixivParser;

impl UrlParser for PixivParser {
    fn from_url(&self, url: String) -> Option<Scraper> {
        let re = Regex::new("https:\\/\\/www.pixiv.net\\/artworks\\/([0-9]+)").unwrap();
        let caps = re.captures(&url);
        if caps.is_none() {
            return None;
        }
        let caps = caps.unwrap();
        if caps.get(1).is_none() {
            return None;
        }
        Some(Scraper::Pixiv(caps.get(1).unwrap().as_str().to_owned()))
    }
}
