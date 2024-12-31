use crate::error::{GCalError, Result};
use crate::event::Event;
use crate::http_client::HttpClient;
use serde_json;

pub struct CalendarClient {
    http_client: HttpClient,
}

impl CalendarClient {
    pub fn new(http_client: HttpClient) -> Self {
        CalendarClient { http_client }
    }

    pub async fn create_event(&self, event: Event) -> Result<Event> {
        // バリデーション
        event
            .validate()
            .map_err(GCalError::ValidationError)?;

        // HTTPクライアントでPOST
        #[cfg(test)]
        let resp = self.http_client.mock_post_response("events", &event).await?;
        #[cfg(not(test))]
        let resp = self.http_client.post("events", &event).await?;

        // レスポンスをEvent構造体にデシリアライズ
        let created_event: Event = serde_json::from_str(&resp)?;

        Ok(created_event)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mock::test_utils::create_test_event;

    #[test]
    fn test_new_calendar_client() {
        let http_client = HttpClient::mock().expect("failed to create mock client");
        let client = CalendarClient::new(http_client);
        assert!(client.http_client.base_url().contains("calendar"));
    }

    #[tokio::test]
    async fn test_create_event_ok() {
        let http_client = HttpClient::mock().expect("failed to create mock client");
        let client = CalendarClient::new(http_client);
        let event = create_test_event();
        let result = client.create_event(event).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_event_validation_error() {
        let http_client = HttpClient::mock().expect("failed to create mock client");
        let client = CalendarClient::new(http_client);

        let event = Event {
            id: None,
            status: None,
            summary: None, // バリデーションエラーの原因
            description: None,
            location: None,
            start: None,
            end: None,
        };

        let result = client.create_event(event).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), GCalError::ValidationError(_)));
    }
}
