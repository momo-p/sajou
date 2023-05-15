use super::{Artwork, FetchError, Gallery, Image, Video};
use curl::easy::{Easy, List};

pub async fn artwork_to_vec(work: &Artwork) -> Result<Vec<u8>, FetchError> {
    match work {
        Artwork::Image(image) => image_to_vec(image).await,
        Artwork::Video(video) => video_to_vec(video).await,
    }
}

async fn image_to_vec(image: &Image) -> Result<Vec<u8>, FetchError> {
    let fetch_url = image.url.clone();
    let mut data = Vec::new();
    let mut handle = Easy::new();
    handle.url(&fetch_url)?;
    add_pixiv_request_hook(&mut handle, &fetch_url)?;
    {
        let mut transfer = handle.transfer();
        transfer.write_function(|new_data| {
            data.extend_from_slice(new_data);
            Ok(new_data.len())
        })?;
        transfer.perform()?;
    }
    match handle.response_code()? {
        200 => Ok(data),
        404 => Err(FetchError::NotFound()),
        _ => Err(FetchError::Other(format!(
            "fetching image {} failed with status code {}",
            &image.url,
            handle.response_code()?
        ))),
    }
}

async fn video_to_vec(video: &Video) -> Result<Vec<u8>, FetchError> {
    todo!()
}

pub async fn gallery_to_vec_on_parallel(
    gallery: &Gallery,
    workers_count: usize,
) -> Result<Vec<u8>, FetchError> {
    todo!()
}

pub async fn gallery_to_vec(gallery: &Gallery) -> Result<Vec<u8>, FetchError> {
    gallery_to_vec_on_parallel(gallery, 1).await
}

fn add_pixiv_request_hook(handle: &mut Easy, url: &str) -> Result<(), FetchError> {
    if !url.to_owned().starts_with("https://i.pximg.net/") {
        return Ok(());
    }

    const USER_AGENT: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.164 Safari/537.36 (c31e105e-822e-4567-87d4-22e0af7a135d)";
    const REFERER: &str = "https://pixiv.net/";
    let mut headers = List::new();
    headers.append(format!("User-Agent: {}", USER_AGENT).as_ref())?;
    headers.append(format!("Referer: {}", REFERER).as_ref())?;
    handle.http_headers(headers)?;
    Ok(())
}
