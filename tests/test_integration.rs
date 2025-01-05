use chrono::{Duration, Utc};
use rust_template::{
    config::GCalConfig,
    calendar_client::CalendarClient,
    event::Event,
    http_client::HttpClient,
};

#[tokio::test]
async fn test_create_event_in_real_api() {
    // 1. Load credentials from "GOOGLE_CREDENTIAL"
    let config = GCalConfig::from_env("GOOGLE_CREDENTIAL")
        .expect("Failed to create config from GOOGLE_CREDENTIAL");

    // 2. Initialize HttpClient with config
    let http_client = HttpClient::new(config).expect("Failed to create HttpClient");

    // 3. Create CalendarClient
    let calendar_client = CalendarClient::new(http_client);

    // 4. Prepare an Event instance
    let start_time = Utc::now() + Duration::hours(1);
    let end_time = start_time + Duration::hours(1);

    let event = Event::new(
        "Integration Test Event".to_string(),
        start_time,
        end_time,
        Some("This event was created by an integration test".to_string()),
        Some("Online".to_string()),
    );

    // 5. Create event in the specified Calendar
    let calendar_id = "5e36da154f2b36a2ad38a79ef17b50626677582ca51d69bf8461003a4a90e20b@group.calendar.google.com";
    let created_event = calendar_client
        .create_event(calendar_id, &event)
        .await
        .expect("Event creation failed");

    // 6. Assert key fields
    assert_eq!(
        created_event.summary,
        Some("Integration Test Event".to_string())
    );
    assert_eq!(
        created_event.description,
        Some("This event was created by an integration test".to_string())
    );
    assert_eq!(created_event.location, Some("Online".to_string()));

    // Event should have an ID after creation
    assert!(
        created_event.id.is_some(),
        "Created event should have an ID"
    );

    // Print event ID for reference
    println!("Created event ID: {:?}", created_event.id);
}
