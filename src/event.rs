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
    date_time: String,
    #[serde(rename = "timeZone")]
    time_zone: String,
}

impl Event {
    pub fn new(
        summary: String,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
        description: Option<String>,
        location: Option<String>,
    ) -> Self {
        Event {
            id: None,
            status: None,
            summary: Some(summary),
            description,
            location,
            start: Some(EventDateTime {
                date_time: start.format("%Y-%m-%dT%H:%M:%SZ").to_string(),
                time_zone: "UTC".to_string(),
            }),
            end: Some(EventDateTime {
                date_time: end.format("%Y-%m-%dT%H:%M:%SZ").to_string(),
                time_zone: "UTC".to_string(),
            }),
        }
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
}
