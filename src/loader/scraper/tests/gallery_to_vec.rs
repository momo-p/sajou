use crate::loader::scraper::{Artwork, Image};
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
