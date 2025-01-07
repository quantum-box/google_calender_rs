use crate::config::GCalConfig;
use crate::error::{GCalError, Result};
use chrono::{Duration as ChronoDuration, Utc};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use reqwest::{Client, Response};
use serde::{Deserialize, Serialize};
use std::time::Duration;

pub struct HttpClient {
    client: Client,
    config: GCalConfig,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    iss: String,
    scope: String,
    aud: String,
    exp: i64,
    iat: i64,
}

impl HttpClient {
    async fn get_access_token(&self) -> Result<String> {
        if let Some(creds_str) = &self.config.credentials {
            let creds: serde_json::Value = serde_json::from_str(creds_str)?;

            let now = Utc::now();
            let claims = Claims {
                iss: creds["client_email"]
                    .as_str()
                    .unwrap_or_default()
                    .to_string(),
                scope: "https://www.googleapis.com/auth/calendar".to_string(),
                aud: "https://oauth2.googleapis.com/token".to_string(),
                exp: (now + ChronoDuration::hours(1)).timestamp(),
                iat: now.timestamp(),
            };

            let private_key = creds["private_key"].as_str().unwrap_or_default();
            let key = EncodingKey::from_rsa_pem(private_key.as_bytes())
                .map_err(|e| GCalError::AuthError(e.to_string()))?;
            let header = Header::new(Algorithm::RS256);
            let jwt =
                encode(&header, &claims, &key).map_err(|e| GCalError::AuthError(e.to_string()))?;

            // Exchange JWT for access token
            let params = [
                ("grant_type", "urn:ietf:params:oauth:grant-type:jwt-bearer"),
                ("assertion", &jwt),
            ];

            let client = Client::new();
            let resp = client
                .post("https://oauth2.googleapis.com/token")
                .form(&params)
                .send()
                .await?
                .json::<serde_json::Value>()
                .await?;

            if let Some(access_token) = resp["access_token"].as_str() {
                Ok(access_token.to_string())
            } else {
                Err(GCalError::AuthError(
                    "Failed to get access token".to_string(),
                ))
            }
        } else {
            Err(GCalError::AuthError("No credentials provided".to_string()))
        }
    }
    #[cfg(test)]
    pub fn mock() -> Result<Self> {
        let config = GCalConfig {
            api_base_url: "https://www.googleapis.com/calendar/v3".to_string(),
            timeout_seconds: 30,
            credentials: None,
        };
        Ok(Self {
            client: Client::new(),
            config,
        })
    }

    #[cfg(test)]
    pub async fn mock_post_response(
        &self,
        _path: &str,
        json: impl serde::Serialize,
    ) -> Result<String> {
        // モックレスポンスとして、リクエストされたイベントをそのまま返す
        Ok(serde_json::to_string(&json)?)
    }

    #[cfg(test)]
    pub async fn mock_get_response(&self, _path: &str) -> Result<String> {
        // テストイベントをJSONにしたものをレスポンスとして返す
        let mock_event = crate::mock::test_utils::create_test_event();
        Ok(serde_json::to_string(&mock_event)?)
    }

    pub fn base_url(&self) -> &str {
        &self.config.api_base_url
    }

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
        let mut request = self.client.post(&url).json(&json);

        // Add authorization if credentials are available
        if self.config.credentials.is_some() {
            let token = self.get_access_token().await?;
            println!("Using authorization token: {}", token);
            request = request.header("Authorization", format!("Bearer {}", token));
        } else {
            println!("No credentials available for authorization");
        }

        println!("Sending request to URL: {}", url);
        let resp = request.send().await?;
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
