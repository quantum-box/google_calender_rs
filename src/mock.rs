#[cfg(test)]
pub mod test_utils {
    use crate::event::Event;
    use chrono::{Duration, Utc};

    pub fn create_test_event() -> Event {
        let now = Utc::now();
        Event {
            id: None,
            status: Some("confirmed".to_string()),
            summary: Some("テスト会議".to_string()),
            description: Some("テストの説明".to_string()),
            location: None,
            start: Some(now),
            end: Some(now + Duration::hours(1)),
        }
    }
}
