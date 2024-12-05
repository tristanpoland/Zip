pub mod dom;
pub mod html;
pub mod css;
pub mod layout;
pub mod renderer;
pub mod network;

use thiserror::Error;
use url::Url;

#[derive(Error, Debug)]
pub enum BrowserError {
    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),
    #[error("HTML parsing error: {0}")]
    HTMLError(String),
    #[error("CSS parsing error: {0}")]
    CSSError(String),
    #[error("URL parsing error: {0}")]
    URLError(#[from] url::ParseError),
    #[error("I/O error: {0}")]
    IOError(#[from] std::io::Error),
}