use chrono::{Duration, Utc};
use rust_template::{
    calendar_client::CalendarClient, config::GCalConfig, event::Event, http_client::HttpClient,
};

const CALENDAR_ID: &str =
    "5e36da154f2b36a2ad38a79ef17b50626677582ca51d69bf8461003a4a90e20b@group.calendar.google.com";

async fn setup_calendar_client() -> CalendarClient {
    let config = GCalConfig::from_env("GOOGLE_SA_SHEET_CRED")
        .expect("Failed to create config from GOOGLE_SA_SHEET_CRED");
    let http_client = HttpClient::new(config).expect("Failed to create HttpClient");
    CalendarClient::new(http_client)
}

#[tokio::test]
async fn test_create_event_in_real_api() {
    let calendar_client = setup_calendar_client().await;

    // UTCイベントの作成
    let start_time = Utc::now() + Duration::hours(1);
    let end_time = start_time + Duration::hours(1);

    let event = Event::new(
        "Integration Test Event (UTC)".to_string(),
        start_time,
        end_time,
        Some("This event was created by an integration test".to_string()),
        Some("Online".to_string()),
        None, // デフォルトでUTC
    )
    .expect("Failed to create event");

    let created_event = calendar_client
        .create_event(CALENDAR_ID, &event)
        .await
        .expect("Event creation failed");

    // UTCイベントの検証
    assert_eq!(
        created_event.summary,
        Some("Integration Test Event (UTC)".to_string())
    );
    assert_eq!(
        created_event.description,
        Some("This event was created by an integration test".to_string())
    );
    assert_eq!(created_event.location, Some("Online".to_string()));
    assert!(
        created_event.id.is_some(),
        "Created event should have an ID"
    );
    assert_eq!(created_event.start.unwrap().time_zone, "UTC");
    assert_eq!(created_event.end.unwrap().time_zone, "UTC");

    println!("Created UTC event ID: {:?}", created_event.id);
}

#[tokio::test]
async fn test_create_event_with_tokyo_timezone() {
    let calendar_client = setup_calendar_client().await;

    // 東京タイムゾーンでのイベント作成
    let start_time = Utc::now() + Duration::hours(1);
    let end_time = start_time + Duration::hours(1);

    let event = Event::new(
        "Integration Test Event (Tokyo)".to_string(),
        start_time,
        end_time,
        Some("This event was created with Asia/Tokyo timezone".to_string()),
        Some("Tokyo".to_string()),
        Some("Asia/Tokyo".to_string()),
    )
    .expect("Failed to create event");

    let created_event = calendar_client
        .create_event(CALENDAR_ID, &event)
        .await
        .expect("Event creation failed");

    // 東京タイムゾーンイベントの検証
    assert_eq!(
        created_event.summary,
        Some("Integration Test Event (Tokyo)".to_string())
    );
    assert_eq!(
        created_event.description,
        Some("This event was created with Asia/Tokyo timezone".to_string())
    );
    assert_eq!(created_event.location, Some("Tokyo".to_string()));
    assert!(
        created_event.id.is_some(),
        "Created event should have an ID"
    );
    assert_eq!(created_event.start.unwrap().time_zone, "Asia/Tokyo");
    assert_eq!(created_event.end.unwrap().time_zone, "Asia/Tokyo");

    println!("Created Tokyo timezone event ID: {:?}", created_event.id);
}
