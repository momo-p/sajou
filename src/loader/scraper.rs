use curl::{easy::Easy, Error as CurlError, FormError as CurlFormError};
use derive_more::Display;
use regex::Error as RegexError;
use scraper::error::SelectorErrorKind;
use serde::{Deserialize, Serialize};
use serde_json::Error as SerdeJsonError;
use std::{convert, str, sync::Arc};

mod gallery_to_vec;
mod pixiv;
mod tests;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Image {
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Video {
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Artwork {
    Image(Image),
    Video(Video),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Gallery {
    pub works: Vec<Artwork>,
}

impl Artwork {
    pub async fn to_vec(&self) -> Result<Vec<u8>, FetchError> {
        gallery_to_vec::artwork_to_vec(&self).await
    }
}

impl Gallery {
    pub async fn to_vec_on_parallel(
        &self,
        workers_count: usize,
    ) -> Result<Vec<Vec<u8>>, FetchError> {
        gallery_to_vec::gallery_to_vec_on_parallel(&self, workers_count).await
    }

    pub async fn to_vec(&self) -> Result<Vec<Vec<u8>>, FetchError> {
        gallery_to_vec::gallery_to_vec(&self).await
    }
}

#[derive(Debug, Clone, Display)]
pub enum FetchError {
    CurlError(CurlError),
    Utf8Error(str::Utf8Error),
    SelectorErrorKind(String),
    RegexError(RegexError),
    SiteStructureChanged(String),
    CurlFormError(CurlFormError),
    SerdeJsonError(Arc<SerdeJsonError>),
    NotFound(),
    Other(String),
}

pub enum Scraper {
    Pixiv(String),
}

impl Scraper {
    pub async fn fetch(&self) -> Result<Gallery, FetchError> {
        match self {
            Scraper::Pixiv(id) => pixiv::fetch(id.to_owned()).await,
        }
    }
}

impl convert::From<CurlError> for FetchError {
    fn from(err: CurlError) -> Self {
        FetchError::CurlError(err)
    }
}

impl convert::From<SelectorErrorKind<'_>> for FetchError {
    fn from(err: SelectorErrorKind) -> Self {
        FetchError::SelectorErrorKind(err.to_string())
    }
}

impl convert::From<str::Utf8Error> for FetchError {
    fn from(err: str::Utf8Error) -> Self {
        FetchError::Utf8Error(err)
    }
}

impl convert::From<RegexError> for FetchError {
    fn from(err: RegexError) -> Self {
        FetchError::RegexError(err)
    }
}

impl convert::From<CurlFormError> for FetchError {
    fn from(err: CurlFormError) -> Self {
        FetchError::CurlFormError(err)
    }
}

impl convert::From<SerdeJsonError> for FetchError {
    fn from(err: SerdeJsonError) -> Self {
        FetchError::SerdeJsonError(Arc::new(err))
    }
}
