use crate::loader::scraper::Scraper;

#[test]
fn parse_pixiv_artwork_url() {
    assert_eq!(
        Scraper::from_url("https://www.pixiv.net/artworks/51363921".to_owned()).unwrap(),
        Scraper::Pixiv("51363921".to_owned())
    )
}

#[test]
fn parse_pixiv_artwork_url_with_trailing() {
    assert_eq!(
        Scraper::from_url("https://www.pixiv.net/artworks/51363921/unknown".to_owned()).unwrap(),
        Scraper::Pixiv("51363921".to_owned())
    )
}

#[test]
fn parse_pixiv_artwork_localized_url() {
    assert_eq!(
        Scraper::from_url("https://www.pixiv.net/en/artworks/51363921/unknown".to_owned()).unwrap(),
        Scraper::Pixiv("51363921".to_owned())
    )
}

#[test]
fn parse_gentoo_org() {
    assert_eq!(
        Scraper::from_url("https://www.gentoo.org/get-started/".to_owned()),
        None
    )
}
