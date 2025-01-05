pub struct GCalConfig {
    pub api_base_url: String,
    pub timeout_seconds: u64,
    pub credentials: Option<String>,
}

impl GCalConfig {
    pub fn from_env(var_name: &str) -> crate::error::Result<Self> {
        let credentials = std::env::var(var_name).map_err(|e| {
            crate::error::GCalError::Other(format!("Failed to read credentials: {}", e))
        })?;

        Ok(Self {
            api_base_url: "https://www.googleapis.com/calendar/v3".to_string(),
            timeout_seconds: 30,
            credentials: Some(credentials),
        })
    }
}

impl Default for GCalConfig {
    fn default() -> Self {
        Self {
            api_base_url: "https://www.googleapis.com/calendar/v3".to_string(),
            timeout_seconds: 30,
            credentials: None,
        }
    }
}

impl GCalConfig {
    pub fn with_base_url(mut self, url: impl Into<String>) -> Self {
        self.api_base_url = url.into();
        self
    }

    pub fn with_timeout(mut self, seconds: u64) -> Self {
        self.timeout_seconds = seconds;
        self
    }
}
