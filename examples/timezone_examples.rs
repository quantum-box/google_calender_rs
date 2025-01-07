use chrono::{Duration, Utc};
use rust_template::{
    calendar_client::CalendarClient, config::GCalConfig, event::Event, http_client::HttpClient,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Google Calendar APIの設定
    let config = GCalConfig::from_env("GOOGLE_SA_SHEET_CRED")?;
    let http_client = HttpClient::new(config)?;
    let calendar_client = CalendarClient::new(http_client);

    // カレンダーIDの設定
    let calendar_id = "5e36da154f2b36a2ad38a79ef17b50626677582ca51d69bf8461003a4a90e20b@group.calendar.google.com";

    // 1. UTCでのイベント作成例
    println!("1. UTCでのイベント作成");
    let start_time = Utc::now() + Duration::hours(1);
    let end_time = start_time + Duration::hours(2);

    let utc_event = Event::new(
        "UTCイベント".to_string(),
        start_time,
        end_time,
        Some("UTCタイムゾーンでのイベント".to_string()),
        Some("オンライン".to_string()),
        None, // タイムゾーン未指定の場合はUTCがデフォルト
    )?;

    let created_utc_event = calendar_client
        .create_event(calendar_id, &utc_event)
        .await?;
    println!("UTCイベントが作成されました: {:?}", created_utc_event.id);

    // 2. 東京タイムゾーンでのイベント作成例
    println!("\n2. 東京タイムゾーンでのイベント作成");
    let tokyo_event = Event::new(
        "東京タイムゾーンイベント".to_string(),
        start_time,
        end_time,
        Some("Asia/Tokyoタイムゾーンでのイベント".to_string()),
        Some("東京".to_string()),
        Some("Asia/Tokyo".to_string()), // Region/City形式でのタイムゾーン指定
    )?;

    let created_tokyo_event = calendar_client
        .create_event(calendar_id, &tokyo_event)
        .await?;
    println!(
        "東京タイムゾーンイベントが作成されました: {:?}",
        created_tokyo_event.id
    );

    // 3. GMTオフセットでのイベント作成例
    println!("\n3. GMTオフセットでのイベント作成");
    let gmt_event = Event::new(
        "GMTオフセットイベント".to_string(),
        start_time,
        end_time,
        Some("GMTオフセットでのイベント".to_string()),
        Some("大阪".to_string()),
        Some("GMT+09:00".to_string()), // GMTオフセット形式でのタイムゾーン指定
    )?;

    let created_gmt_event = calendar_client
        .create_event(calendar_id, &gmt_event)
        .await?;
    println!(
        "GMTオフセットイベントが作成されました: {:?}",
        created_gmt_event.id
    );

    Ok(())
}
