use super::{Artwork, FetchError, Gallery, Image};
use crate::context::BotContextInterface;
use curl::easy::{Easy, List};
use serde::Deserialize;
use std::{str, sync::Arc};

const USER_AGENT: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.164 Safari/537.36 (c31e105e-822e-4567-87d4-22e0af7a135d)";
const REFERER: &str = "https://pixiv.net/";
const PIXIV_SSID_COOKIE_NAME: &str = "PHPSESSID";

pub async fn fetch_with_context(
    id: String,
    context: Option<Arc<BotContextInterface>>,
) -> Result<Gallery, FetchError> {
    let response = get_pages(id, context).await?;
    Ok(Gallery {
        works: response
            .body
            .iter()
            .map(|content| {
                Artwork::Image(Image {
                    url: content.urls.original.clone(),
                })
            })
            .collect::<Vec<_>>(),
    })
}

pub async fn fetch(id: String) -> Result<Gallery, FetchError> {
    fetch_with_context(id, None).await
}

fn get_headers(context: Option<Arc<BotContextInterface>>) -> Result<List, FetchError> {
    let mut headers = List::new();
    headers.append(format!("User-Agent: {}", USER_AGENT).as_ref())?;
    headers.append(format!("Referer: {}", REFERER).as_ref())?;
    if context.is_some() {
        let context = context.unwrap();
        headers.append(
            format!(
                "Cookie: {}={};",
                PIXIV_SSID_COOKIE_NAME,
                context.clone().config.pixiv_ssid,
            )
            .as_ref(),
        )?;
    }
    Ok(headers)
}

async fn get_pages(
    id: String,
    context: Option<Arc<BotContextInterface>>,
) -> Result<Response, FetchError> {
    let fetch_url = format!("https://www.pixiv.net/ajax/illust/{}/pages?lang=en", id);
    let mut data = Vec::new();
    let mut handle = Easy::new();
    handle.url(&fetch_url)?;
    handle.http_headers(get_headers(context)?)?;
    {
        let mut transfer = handle.transfer();
        transfer.write_function(|new_data| {
            data.extend_from_slice(new_data);
            Ok(new_data.len())
        })?;
        transfer.perform()?;
    }
    let body = str::from_utf8(&mut data)?.to_owned();
    let response: Response = serde_json::from_str(&body)?;
    if response.error {
        if response.message.clone() == "Couldn't find requested page" {
            return Err(FetchError::NotFound());
        }
        return Err(FetchError::Other(response.message));
    }
    Ok(response)
}

#[derive(Debug, Clone, Deserialize)]
struct Response {
    pub error: bool,
    pub message: String,
    pub body: Vec<Content>,
}

#[derive(Debug, Clone, Deserialize)]
struct Content {
    pub urls: Url,
}

#[derive(Debug, Clone, Deserialize)]
struct Url {
    pub original: String,
}
