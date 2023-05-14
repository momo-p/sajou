use curl::{easy::Easy, Error as CurlError, FormError as CurlFormError};
use derive_more::Display;
use regex::Error as RegexError;
use scraper::{error::SelectorErrorKind, html::Html};
use serde::{Deserialize, Serialize};
use serde_json::Error as SerdeJsonError;
use std::{convert, str};

mod pixiv;

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

#[derive(Debug, Display)]
pub enum FetchError {
    CurlError(CurlError),
    Utf8Error(str::Utf8Error),
    SelectorErrorKind(String),
    RegexError(RegexError),
    SiteStructureChanged(String),
    CurlFormError(CurlFormError),
    SerdeJsonError(SerdeJsonError),
    NotFound(),
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
        FetchError::SerdeJsonError(err)
    }
}
