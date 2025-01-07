#[cfg(test)]
pub mod test_utils {
    use crate::event::Event;
    use chrono::{Duration, Utc};

    pub fn create_test_event() -> Event {
        let now = Utc::now();
        Event::new(
            "テスト会議".to_string(),
            now,
            now + Duration::hours(1),
            Some("テストの説明".to_string()),
            None,
            None, // デフォルトでUTCを使用
        )
        .expect("テストイベントの作成に失敗")
    }
}
