#[cfg(test)]
mod tests {
    use crate::types::announcement::*;
    use crate::types::auth::*;
    use crate::types::device::*;
    use crate::types::ws::*;
    use chrono::Utc;
    use uuid::Uuid;

    #[test]
    fn test_announcement_serialization_roundtrip() {
        let ann = Announcement {
            id: Uuid::new_v4(),
            title: "测试公告".to_string(),
            content_html: "<b>粗体</b>".to_string(),
            announcement_type: AnnouncementType::Normal,
            publisher_name: "张三".to_string(),
            publisher_user_id: "user_abc".to_string(),
            created_at: Utc::now(),
            updated_at: None,
        };
        let json = serde_json::to_string(&ann).unwrap();
        let decoded: Announcement = serde_json::from_str(&json).unwrap();
        assert_eq!(ann.id, decoded.id);
        assert_eq!(ann.title, decoded.title);
        assert_eq!(ann.content_html, decoded.content_html);
    }

    #[test]
    fn test_announcement_type_display() {
        assert_eq!(AnnouncementType::Normal.to_string(), "normal");
        assert_eq!(AnnouncementType::Emergency.to_string(), "emergency");
        assert_eq!(AnnouncementType::Notice.to_string(), "notice");
    }

    #[test]
    fn test_announcement_type_serde() {
        let normal = AnnouncementType::Normal;
        let json = serde_json::to_string(&normal).unwrap();
        assert_eq!(json, r#""normal""#);

        let emergency = AnnouncementType::Emergency;
        let json = serde_json::to_string(&emergency).unwrap();
        assert_eq!(json, r#""emergency""#);

        let decoded: AnnouncementType = serde_json::from_str(r#""notice""#).unwrap();
        assert_eq!(decoded, AnnouncementType::Notice);
    }

    #[test]
    fn test_emergency_type_roundtrip() {
        let ann = Announcement {
            id: Uuid::new_v4(),
            title: "紧急通知".to_string(),
            content_html: "紧急内容".to_string(),
            announcement_type: AnnouncementType::Emergency,
            publisher_name: "李四".to_string(),
            publisher_user_id: "user_def".to_string(),
            created_at: Utc::now(),
            updated_at: None,
        };
        let json = serde_json::to_string(&ann).unwrap();
        let decoded: Announcement = serde_json::from_str(&json).unwrap();
        assert_eq!(decoded.announcement_type, AnnouncementType::Emergency);
    }

    #[test]
    fn test_auth_token_serde() {
        let token = AuthToken {
            access_token: "eyJhbGciOi...".to_string(),
            refresh_token: "refresh_xxx".to_string(),
            token_type: "Bearer".to_string(),
            expires_in: 3600,
            user_id: "user_abc".to_string(),
        };
        let json = serde_json::to_string(&token).unwrap();
        let decoded: AuthToken = serde_json::from_str(&json).unwrap();
        assert_eq!(token, decoded);
    }

    #[test]
    fn test_user_info_serde() {
        let info = UserInfo {
            user_id: "user_abc".to_string(),
            user_name: "张三".to_string(),
            email: "zhangsan@example.com".to_string(),
            permission: 10,
        };
        let json = serde_json::to_string(&info).unwrap();
        let decoded: UserInfo = serde_json::from_str(&json).unwrap();
        assert_eq!(info, decoded);
    }

    #[test]
    fn test_token_status_serde() {
        let status = TokenStatus {
            active: true,
            user_id: "user_abc".to_string(),
            client_id: "pf_xxx".to_string(),
            expires_at: Some("2026-01-01T00:00:00Z".to_string()),
            scope: Some("user:read".to_string()),
        };
        let json = serde_json::to_string(&status).unwrap();
        let decoded: TokenStatus = serde_json::from_str(&json).unwrap();
        assert_eq!(status, decoded);
    }

    #[test]
    fn test_device_info_serde() {
        let device = DeviceInfo {
            device_uuid: Uuid::new_v4().to_string(),
            device_name: "教室大屏-101".to_string(),
            platform: Platform::Windows,
        };
        let json = serde_json::to_string(&device).unwrap();
        let decoded: DeviceInfo = serde_json::from_str(&json).unwrap();
        assert_eq!(device, decoded);
    }

    #[test]
    fn test_platform_detect() {
        let platform = Platform::detect();
        assert!(matches!(platform, Platform::Windows | Platform::Linux | Platform::MacOS));
    }

    #[test]
    fn test_ws_message_serde() {
        let ann = Announcement {
            id: Uuid::new_v4(),
            title: "测试".to_string(),
            content_html: "<p>test</p>".to_string(),
            announcement_type: AnnouncementType::Normal,
            publisher_name: "测试员".to_string(),
            publisher_user_id: "u1".to_string(),
            created_at: Utc::now(),
            updated_at: None,
        };
        let msg = WsMessage::ServerPush { announcement: ann };
        let json = serde_json::to_string(&msg).unwrap();
        let decoded: WsMessage = serde_json::from_str(&json).unwrap();
        match decoded {
            WsMessage::ServerPush { announcement } => {
                assert_eq!(announcement.title, "测试");
            }
            _ => panic!("Expected ServerPush"),
        }
    }

    #[test]
    fn test_ws_message_ack() {
        let msg = WsMessage::ClientAck {
            message_id: "msg_001".to_string(),
        };
        let json = serde_json::to_string(&msg).unwrap();
        assert!(json.contains("client_ack"));
        let decoded: WsMessage = serde_json::from_str(&json).unwrap();
        assert!(matches!(decoded, WsMessage::ClientAck { .. }));
    }

    #[test]
    fn test_ws_message_heartbeat() {
        let msg = WsMessage::Heartbeat { timestamp: 1234567890 };
        let json = serde_json::to_string(&msg).unwrap();
        let decoded: WsMessage = serde_json::from_str(&json).unwrap();
        assert!(matches!(decoded, WsMessage::Heartbeat { .. }));
    }

    #[test]
    fn test_announcement_create_serde() {
        let create = AnnouncementCreate {
            title: "新建公告".to_string(),
            content_html: "<p>内容</p>".to_string(),
            announcement_type: AnnouncementType::Notice,
            publisher_name: "发布者".to_string(),
        };
        let json = serde_json::to_string(&create).unwrap();
        let decoded: AnnouncementCreate = serde_json::from_str(&json).unwrap();
        assert_eq!(decoded.title, "新建公告");
    }

    #[test]
    fn test_announcement_list_response() {
        let response = AnnouncementListResponse {
            announcements: vec![],
            total: 0,
            page: 1,
            per_page: 20,
        };
        let json = serde_json::to_string(&response).unwrap();
        let decoded: AnnouncementListResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(decoded.total, 0);
    }
}
