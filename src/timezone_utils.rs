use chrono::{DateTime, Utc};
use std::str::FromStr;

/// タイムゾーン変換に関するエラー
#[derive(Debug)]
pub enum TimezoneError {
    /// 無効なタイムゾーン文字列
    InvalidTimezone(String),
    /// 日時変換エラー
    ConversionError(String),
}

impl std::fmt::Display for TimezoneError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TimezoneError::InvalidTimezone(tz) => write!(f, "無効なタイムゾーン文字列です: {}", tz),
            TimezoneError::ConversionError(msg) => write!(f, "日時変換エラー: {}", msg),
        }
    }
}

impl std::error::Error for TimezoneError {}

/// タイムゾーン文字列が有効かどうかを検証します
pub fn validate_timezone(tz: &str) -> bool {
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

/// UTCの日時を指定されたタイムゾーンに変換します
pub fn convert_to_timezone(dt: DateTime<Utc>, timezone: &str) -> Result<String, TimezoneError> {
    if !validate_timezone(timezone) {
        return Err(TimezoneError::InvalidTimezone(timezone.to_string()));
    }

    // UTCの場合は直接フォーマット
    if timezone == "UTC" {
        return Ok(dt.format("%Y-%m-%dT%H:%M:%SZ").to_string());
    }

    // GMT+/-XX:XX形式の場合は、オフセットを解析して適用
    if timezone.starts_with("GMT") {
        let offset = &timezone[3..];
        if let Ok(fixed_offset) = chrono::FixedOffset::from_str(offset) {
            let local_dt = dt.with_timezone(&fixed_offset);
            return Ok(local_dt.format("%Y-%m-%dT%H:%M:%S%:z").to_string());
        }
    }

    // Region/City形式の場合は、そのままGoogle Calendar APIが解釈できる形式で返す
    Ok(dt.format("%Y-%m-%dT%H:%M:%S").to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_validate_timezone() {
        assert!(validate_timezone("UTC"));
        assert!(validate_timezone("Asia/Tokyo"));
        assert!(validate_timezone("GMT+09:00"));
        assert!(!validate_timezone("Invalid/Zone"));
        assert!(!validate_timezone("GMT+9"));
    }

    #[test]
    fn test_convert_to_timezone() {
        let dt = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();

        // UTCの場合
        let result = convert_to_timezone(dt, "UTC").unwrap();
        assert!(result.ends_with("Z"));

        // GMT+09:00の場合
        let result = convert_to_timezone(dt, "GMT+09:00").unwrap();
        assert!(result.contains("+09:00"));

        // Asia/Tokyoの場合
        let result = convert_to_timezone(dt, "Asia/Tokyo").unwrap();
        assert!(!result.contains("Z")); // タイムゾーン情報なし（Google Calendar APIが解釈）
    }

    #[test]
    fn test_invalid_timezone() {
        let dt = Utc::now();
        assert!(convert_to_timezone(dt, "Invalid/Zone").is_err());
    }
}
