use chrono::{DateTime, Utc};

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
        if parts.len() == 2 {
            let region = parts[0];
            let city = parts[1];
            // 有効な地域名のチェック
            let valid_regions = [
                "Asia",
                "America",
                "Europe",
                "Africa",
                "Australia",
                "Pacific",
                "Atlantic",
                "Indian",
            ];
            if valid_regions.contains(&region) && !city.is_empty() {
                return true;
            }
        }
        return false;
    }

    // GMT+/-XX:XX 形式
    if let Some(offset) = tz.strip_prefix("GMT") {
        // +09:00 形式のチェック
        if offset.len() == 6
            && (offset.starts_with('+') || offset.starts_with('-'))
            && offset[4..5].contains(':')
            && offset[1..3].chars().all(|c| c.is_ascii_digit())
            && offset[5..].chars().all(|c| c.is_ascii_digit())
        {
            if let (Ok(hours), Ok(minutes)) = (offset[1..3].parse::<i32>(), offset[5..].parse::<i32>()) {
                return (0..=23).contains(&hours) && (0..=59).contains(&minutes);
            }
        }
        return false;
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
    if let Some(offset) = timezone.strip_prefix("GMT") {
        // +09:00 形式のチェック
        if offset.len() == 6
            && (offset.starts_with('+') || offset.starts_with('-'))
            && offset[4..5].contains(':')
            && offset[1..3].chars().all(|c| c.is_ascii_digit())
            && offset[5..].chars().all(|c| c.is_ascii_digit())
        {
            let hours = offset[1..3].parse::<i32>().unwrap_or(24);
            let minutes = offset[5..].parse::<i32>().unwrap_or(60);
            if (0..=23).contains(&hours) && (0..=59).contains(&minutes) {
                let local_dt = dt.format("%Y-%m-%dT%H:%M:%S").to_string();
                return Ok(format!("{}{}", local_dt, offset));
            }
        }
        return Err(TimezoneError::InvalidTimezone(timezone.to_string()));
    }

    // Region/City形式の場合は、そのままGoogle Calendar APIが解釈できる形式で返す
    if timezone.contains('/') {
        let parts: Vec<&str> = timezone.split('/').collect();
        if parts.len() == 2 && !parts[0].is_empty() && !parts[1].is_empty() {
            return Ok(dt.format("%Y-%m-%dT%H:%M:%S").to_string());
        }
    }
    Err(TimezoneError::InvalidTimezone(timezone.to_string()))
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
