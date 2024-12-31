use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Event {
    pub id: Option<String>,
    pub status: Option<String>,
    pub summary: Option<String>,
    pub description: Option<String>,
    pub location: Option<String>,
    pub start: Option<DateTime<Utc>>,
    pub end: Option<DateTime<Utc>>,
}

impl Event {
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
