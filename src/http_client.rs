use crate::config::GCalConfig;
use crate::error::{GCalError, Result};
use reqwest::{Client, Response};
use std::time::Duration;

pub struct HttpClient {
    client: Client,
    config: GCalConfig,
}

impl HttpClient {
    pub fn new(config: GCalConfig) -> Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(config.timeout_seconds))
            .build()
            .map_err(GCalError::from)?;
        Ok(HttpClient { client, config })
    }

    pub async fn get(&self, path: &str) -> Result<String> {
        let url = format!("{}/{}", self.config.api_base_url, path);
        let resp = self.client.get(&url).send().await?;
        self.handle_response(resp).await
    }

    pub async fn post(&self, path: &str, json: impl serde::Serialize) -> Result<String> {
        let url = format!("{}/{}", self.config.api_base_url, path);
        let resp = self.client.post(&url).json(&json).send().await?;
        self.handle_response(resp).await
    }

    pub async fn put(&self, path: &str, json: impl serde::Serialize) -> Result<String> {
        let url = format!("{}/{}", self.config.api_base_url, path);
        let resp = self.client.put(&url).json(&json).send().await?;
        self.handle_response(resp).await
    }

    pub async fn delete(&self, path: &str) -> Result<String> {
        let url = format!("{}/{}", self.config.api_base_url, path);
        let resp = self.client.delete(&url).send().await?;
        self.handle_response(resp).await
    }

    async fn handle_response(&self, response: Response) -> Result<String> {
        if response.status().is_success() {
            Ok(response.text().await?)
        } else {
            Err(GCalError::Other(format!(
                "APIエラー: ステータスコード {} - {}",
                response.status(),
                response.text().await?
            )))
        }
    }
}
