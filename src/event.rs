use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    pub summary: Option<String>,
    pub description: Option<String>,
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<EventDateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<EventDateTime>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EventDateTime {
    #[serde(rename = "dateTime")]
    pub date_time: String,
    #[serde(rename = "timeZone")]
    pub time_zone: String,
}

impl EventDateTime {
    /// Creates a new EventDateTime with the given datetime string and timezone
    pub fn new(date_time: String, time_zone: String) -> Result<Self, String> {
        // タイムゾーン文字列の検証
        // 一般的なタイムゾーン形式（"UTC", "Asia/Tokyo" など）をチェック
        if !Self::is_valid_timezone(&time_zone) {
            return Err(format!("無効なタイムゾーン文字列です: {}", time_zone));
        }

        Ok(EventDateTime {
            date_time,
            time_zone,
        })
    }

    /// Creates an EventDateTime from a DateTime and timezone string
    pub fn from_datetime_with_tz<Tz: chrono::TimeZone>(
        dt: DateTime<Tz>,
        time_zone: String,
    ) -> Result<Self, String> {
        if !Self::is_valid_timezone(&time_zone) {
            return Err(format!("無効なタイムゾーン文字列です: {}", time_zone));
        }

        Ok(EventDateTime {
            date_time: dt.to_rfc3339(),
            time_zone,
        })
    }

    /// Validates if the given timezone string is valid
    fn is_valid_timezone(tz: &str) -> bool {
        // 基本的なタイムゾーン形式のチェック
        // UTC
        if tz == "UTC" {
            return true;
        }

        // Region/City 形式 (例: "Asia/Tokyo")
        if tz.contains('/') {
            let parts: Vec<&str> = tz.split('/').collect();
            if parts.len() == 2 && !parts[0].is_empty() && !parts[1].is_empty() {
                return true;
            }
        }

        // GMT+/-XX:XX 形式
        if tz.starts_with("GMT") {
            let offset = &tz[3..];
            if offset.starts_with('+') || offset.starts_with('-') {
                if offset.len() == 6 && offset[4..5].contains(':') {
                    return true;
                }
            }
        }

        false
    }
}

impl Event {
    pub fn new(
        summary: String,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
        description: Option<String>,
        location: Option<String>,
        time_zone: Option<String>,
    ) -> Result<Self, String> {
        // タイムゾーンのデフォルト値はUTC
        let tz = time_zone.unwrap_or_else(|| "UTC".to_string());

        // タイムゾーンの検証
        if !EventDateTime::is_valid_timezone(&tz) {
            return Err(format!("無効なタイムゾーン文字列です: {}", tz));
        }

        // start EventDateTimeの作成
        let start_dt = EventDateTime::from_datetime_with_tz(start, tz.clone())?;
        // end EventDateTimeの作成
        let end_dt = EventDateTime::from_datetime_with_tz(end, tz)?;

        Ok(Event {
            id: None,
            status: None,
            summary: Some(summary),
            description,
            location,
            start: Some(start_dt),
            end: Some(end_dt),
        })
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.summary.is_none() {
            return Err("イベントタイトル(summary)が必要です".to_string());
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Duration, Utc};

    #[test]
    fn test_validate_ok() {
        let ev = Event {
            id: None,
            status: Some("confirmed".to_string()),
            summary: Some("テスト会議".to_string()),
            description: None,
            location: None,
            start: None,
            end: None,
        };
        assert!(ev.validate().is_ok());
    }

    #[test]
    fn test_validate_fail() {
        let ev = Event {
            id: None,
            status: None,
            summary: None,
            description: None,
            location: None,
            start: None,
            end: None,
        };
        assert!(ev.validate().is_err());
    }

    #[test]
    fn test_new_with_utc() {
        let start = Utc::now();
        let end = start + Duration::hours(1);
        let event = Event::new(
            "テストイベント".to_string(),
            start,
            end,
            Some("説明".to_string()),
            Some("場所".to_string()),
            None,
        );
        assert!(event.is_ok());
        let event = event.unwrap();
        assert_eq!(event.summary, Some("テストイベント".to_string()));
        assert_eq!(event.start.unwrap().time_zone, "UTC");
        assert_eq!(event.end.unwrap().time_zone, "UTC");
    }

    #[test]
    fn test_new_with_tokyo_timezone() {
        let start = Utc::now();
        let end = start + Duration::hours(1);
        let event = Event::new(
            "テストイベント".to_string(),
            start,
            end,
            Some("説明".to_string()),
            Some("場所".to_string()),
            Some("Asia/Tokyo".to_string()),
        );
        assert!(event.is_ok());
        let event = event.unwrap();
        assert_eq!(event.start.unwrap().time_zone, "Asia/Tokyo");
        assert_eq!(event.end.unwrap().time_zone, "Asia/Tokyo");
    }

    #[test]
    fn test_new_with_invalid_timezone() {
        let start = Utc::now();
        let end = start + Duration::hours(1);
        let event = Event::new(
            "テストイベント".to_string(),
            start,
            end,
            Some("説明".to_string()),
            Some("場所".to_string()),
            Some("Invalid/Timezone".to_string()),
        );
        assert!(event.is_err());
    }
}
