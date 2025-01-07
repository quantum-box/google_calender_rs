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

    pub async fn create_event(&self, calendar_id: &str, event: &Event) -> Result<Event> {
        // バリデーション
        event.validate().map_err(GCalError::ValidationError)?;

        // イベント作成のパスを構築
        let path = format!("calendars/{}/events", calendar_id);

        // Debug: Print the event JSON
        let event_json = serde_json::to_string_pretty(event).unwrap();
        println!("Request payload:\n{}", event_json);

        // HTTPクライアントでPOST
        #[cfg(test)]
        let resp = self.http_client.mock_post_response(&path, event).await?;
        #[cfg(not(test))]
        let resp = self.http_client.post(&path, event).await?;

        // レスポンスをEvent構造体にデシリアライズ
        let created_event: Event = serde_json::from_str(&resp)?;

        Ok(created_event)
    }

    /// 1つのイベントを取得
    pub async fn get_event(&self, calendar_id: &str, event_id: &str) -> Result<Event> {
        // GET /calendars/{calendarId}/events/{eventId} を実行
        let path = format!("calendars/{}/events/{}", calendar_id, event_id);

        // HTTPクライアントでGET
        #[cfg(test)]
        let resp = self.http_client.mock_get_response(&path).await?;
        #[cfg(not(test))]
        let resp = self.http_client.get(&path).await?;

        // レスポンスをEvent構造体にデシリアライズ
        let fetched_event: Event = serde_json::from_str(&resp)?;
        Ok(fetched_event)
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
        let calendar_id = "test_calendar";
        let result = client.create_event(calendar_id, &event).await;
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

        let calendar_id = "test_calendar";
        let result = client.create_event(calendar_id, &event).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), GCalError::ValidationError(_)));
    }

    #[tokio::test]
    async fn test_get_event_ok() {
        let http_client = HttpClient::mock().expect("failed to create mock client");
        let client = CalendarClient::new(http_client);
        let calendar_id = "test_calendar";
        let event_id = "test_event_123";
        let result = client.get_event(calendar_id, event_id).await;
        assert!(result.is_ok());
        let fetched_event = result.unwrap();
        assert_eq!(fetched_event.summary.as_deref(), Some("テスト会議"));
    }
}
