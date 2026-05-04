use cicada_api::rest::ApiClient;
use cicada_core::types::announcement::{AnnouncementCreate, AnnouncementType};
use serde_json::json;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_full_api_flow() {
    let server = MockServer::start().await;
    let client = ApiClient::new(server.uri());

    Mock::given(method("GET"))
        .and(path("/api/v1/announcements"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "announcements": [],
            "total": 0, "page": 1, "per_page": 20
        })))
        .mount(&server)
        .await;

    let list = client.get_announcements(None, None, None).await.unwrap();
    assert_eq!(list.total, 0);

    Mock::given(method("POST"))
        .and(path("/api/v1/announcements"))
        .respond_with(ResponseTemplate::new(201).set_body_json(json!({
            "id": "550e8400-e29b-41d4-a716-446655440002",
            "title": "集成测试",
            "content_html": "<p>integrated</p>",
            "announcement_type": "emergency",
            "publisher_name": "集成测试员",
            "publisher_user_id": "test_user",
            "created_at": "2026-06-15T08:00:00Z",
            "updated_at": null
        })))
        .mount(&server)
        .await;

    let payload = AnnouncementCreate {
        title: "集成测试".to_string(),
        content_html: "<p>integrated</p>".to_string(),
        announcement_type: AnnouncementType::Emergency,
        publisher_name: "集成测试员".to_string(),
    };

    let created = client.create_announcement(&payload, "token").await.unwrap();
    assert_eq!(created.title, "集成测试");
    assert_eq!(created.announcement_type, AnnouncementType::Emergency);
}

#[tokio::test]
async fn test_error_flow() {
    let server = MockServer::start().await;
    let client = ApiClient::new(server.uri());

    Mock::given(method("GET"))
        .and(path("/api/v1/announcements/not-found"))
        .respond_with(ResponseTemplate::new(404))
        .mount(&server)
        .await;

    let result = client.get_announcement("not-found").await;
    assert!(result.is_err());
}
