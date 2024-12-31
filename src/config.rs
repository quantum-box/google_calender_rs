use crate::error::GCalError;
use std::env;

pub struct GCalConfig {
    pub api_base_url: String,
    pub timeout_seconds: u64,
}

impl Default for GCalConfig {
    fn default() -> Self {
        Self {
            api_base_url: "https://www.googleapis.com/calendar/v3".to_string(),
            timeout_seconds: 30,
        }
    }
}

impl GCalConfig {
    pub fn from_env() -> Result<Self, GCalError> {
        let api_base_url =
            env::var("GCAL_API_BASE_URL").unwrap_or_else(|_| Self::default().api_base_url);

        let timeout_seconds = env::var("GCAL_TIMEOUT_SECONDS")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(Self::default().timeout_seconds);

        Ok(Self {
            api_base_url,
            timeout_seconds,
        })
    }

    pub fn with_base_url(mut self, url: impl Into<String>) -> Self {
        self.api_base_url = url.into();
        self
    }

    pub fn with_timeout(mut self, seconds: u64) -> Self {
        self.timeout_seconds = seconds;
        self
    }
}
