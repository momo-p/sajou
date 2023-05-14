use crate::loader::scraper::{Artwork, FetchError, Gallery, Image, Scraper};

macro_rules! aw {
    ($e:expr) => {
        tokio_test::block_on($e)
    };
}

#[test]
fn single_artwork() {
    let response = aw!(Scraper::Pixiv("108027507".to_owned()).fetch()).unwrap();
    assert_eq!(
        response,
        Gallery {
            works: vec![Artwork::Image(Image {
                url:
                    "https://i.pximg.net/img-original/img/2023/05/11/14/11/01/108027507_ugoira0.jpg"
                        .to_owned(),
            }),]
        }
    )
}

#[test]
fn multiple_artwork() {
    let response = aw!(Scraper::Pixiv("107308926".to_owned()).fetch()).unwrap();
    assert_eq!(
        response,
        Gallery {
            works: vec![
                Artwork::Image(Image {
                    url:
                        "https://i.pximg.net/img-original/img/2023/04/19/01/18/34/107308926_p0.png"
                            .to_owned(),
                }),
                Artwork::Image(Image {
                    url:
                        "https://i.pximg.net/img-original/img/2023/04/19/01/18/34/107308926_p1.png"
                            .to_owned(),
                }),
            ]
        }
    )
}

#[test]
fn invalid_id() {
    let response = aw!(Scraper::Pixiv("invalid_id".to_owned()).fetch());
    match response {
        Ok(_) => panic!("should throw an error"),
        Err(err) => match err {
            FetchError::NotFound() => assert_eq!(1, 1),
            _ => panic!("should throw \"not found\", found: {:?}", err),
        },
    };
}
