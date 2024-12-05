// src/network.rs
use reqwest;
use crate::BrowserError;
use url::Url;
use druid::Data;
use std::sync::Arc;

#[derive(Clone)]
pub struct NetworkClient {
    client: Arc<reqwest::Client>,
}

// Manual implementation of Data trait because reqwest::Client doesn't implement Data
impl Data for NetworkClient {
    fn same(&self, _other: &Self) -> bool {
        // Since the client is wrapped in Arc, we consider them the same
        // This is safe because Client is effectively stateless
        true
    }
}

impl NetworkClient {
    pub fn new() -> Self {
        NetworkClient {
            client: Arc::new(reqwest::Client::new()),
        }
    }

    pub async fn fetch(&self, url: &str) -> Result<String, BrowserError> {
        let url = Url::parse(url)?;
        let response = self.client.get(url)
            .send()
            .await?
            .text()
            .await?;
        Ok(response)
    }
}