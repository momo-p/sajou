use crate::loader::scraper::{Artwork, FetchError, Gallery, Image};
use std::fs;

/// Credits for the image goes to: https://www.pixiv.net/artworks/19724696
#[tokio::test]
async fn download_tux_image_as_single_artwork() {
    let artwork = Artwork::Image(Image {
        url: "https://i.pximg.net/img-original/img/2011/06/18/22/55/57/19724696_p0.jpg".to_owned(),
    });
    let image = artwork.to_vec().await.unwrap();
    let target = fs::read("resources/pixiv_tux.jpg").unwrap();
    assert_eq!(image, target);
}

#[tokio::test]
async fn download_invalid_artwork_image() {
    let artwork = Artwork::Image(Image {
        url: "https://i.pximg.net/img-original/img/2011/06/18/22/55/57/tux.jpg".to_owned(),
    });
    let image = artwork.to_vec().await;
    match image {
        Ok(_) => panic!("expect to throw FetchError::NotFound()"),
        Err(err) => match err {
            FetchError::NotFound() => assert_eq!(1, 1),
            _ => panic!("exepect to throw FetchError::NotFound(), found {:?}", err),
        },
    }
}

#[tokio::test]
async fn download_tux_image_as_a_gallery_single_worker() {
    let gallery = Gallery {
        works: vec![Artwork::Image(Image {
            url: "https://i.pximg.net/img-original/img/2011/06/18/22/55/57/19724696_p0.jpg"
                .to_owned(),
        })],
    };
    let image = gallery.to_vec().await.unwrap();
    let target = fs::read("resources/pixiv_tux.jpg").unwrap();
    let target = vec![target.to_vec()];
    assert_eq!(image, target);
}

/// Credits for tux_0.jpg image goes to: https://www.pixiv.net/artworks/51363921
#[tokio::test]
async fn download_multiple_tux_image_as_a_gallery_multiple_worker() {
    let gallery = Gallery {
        works: vec![
            Artwork::Image(Image {
                url: "https://i.pximg.net/img-original/img/2011/06/18/22/55/57/19724696_p0.jpg"
                    .to_owned(),
            }),
            Artwork::Image(Image {
                url: "https://i.pximg.net/img-original/img/2015/07/11/20/56/48/51363921_p0.png"
                    .to_owned(),
            }),
            Artwork::Image(Image {
                url: "https://i.pximg.net/img-original/img/2011/06/18/22/55/57/19724696_p0.jpg"
                    .to_owned(),
            }),
            Artwork::Image(Image {
                url: "https://i.pximg.net/img-original/img/2011/06/18/22/55/57/19724696_p0.jpg"
                    .to_owned(),
            }),
            Artwork::Image(Image {
                url: "https://i.pximg.net/img-original/img/2015/07/11/20/56/48/51363921_p0.png"
                    .to_owned(),
            }),
        ],
    };
    let image = gallery.to_vec().await.unwrap();
    let pixiv_tux = fs::read("resources/pixiv_tux.jpg").unwrap();
    let tux_0 = fs::read("resources/tux_0.jpg").unwrap();
    let target = vec![
        pixiv_tux.to_vec(),
        tux_0.to_vec(),
        pixiv_tux.to_vec(),
        pixiv_tux.to_vec(),
        tux_0.to_vec(),
    ];
    assert_eq!(image, target);
}
