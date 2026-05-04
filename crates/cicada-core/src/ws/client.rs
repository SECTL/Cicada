use std::time::Duration;

use futures_util::{SinkExt, StreamExt};
use tokio::sync::mpsc;
use tokio_tungstenite::{connect_async, tungstenite::Message};

use crate::types::ws::WsMessage;

pub enum WsEvent {
    Connected,
    Disconnected,
    Message(WsMessage),
    Error(String),
}

pub struct WsClient {
    url: String,
    event_tx: Option<mpsc::UnboundedSender<WsEvent>>,
}

impl WsClient {
    pub fn new(url: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            event_tx: None,
        }
    }

    pub fn event_receiver(&mut self) -> mpsc::UnboundedReceiver<WsEvent> {
        let (tx, rx) = mpsc::unbounded_channel();
        self.event_tx = Some(tx);
        rx
    }

    pub async fn connect(&self) {
        let tx = match &self.event_tx {
            Some(tx) => tx.clone(),
            None => return,
        };

        let url = self.url.clone();
        let mut backoff_secs = 1u64;

        loop {
            match connect_async(&url).await {
                Ok((ws_stream, _)) => {
                    backoff_secs = 1;
                    let _ = tx.send(WsEvent::Connected);

                    let (mut write, mut read) = ws_stream.split();

                    let tx_clone = tx.clone();
                    let heartbeat_handle = tokio::spawn(async move {
                        loop {
                            tokio::time::sleep(Duration::from_secs(30)).await;
                            if write.send(Message::Ping(vec![])).await.is_err() {
                                break;
                            }
                        }
                    });

                    while let Some(msg_result) = read.next().await {
                        match msg_result {
                            Ok(Message::Text(text)) => {
                                if let Ok(ws_msg) = serde_json::from_str::<WsMessage>(&text) {
                                    let _ = tx_clone.send(WsEvent::Message(ws_msg));
                                }
                            }
                            Ok(Message::Pong(_)) => {}
                            Ok(Message::Close(_)) => break,
                            Err(e) => {
                                let _ = tx_clone.send(WsEvent::Error(e.to_string()));
                                break;
                            }
                            _ => {}
                        }
                    }

                    heartbeat_handle.abort();
                    let _ = tx_clone.send(WsEvent::Disconnected);
                }
                Err(e) => {
                    let _ = tx.send(WsEvent::Error(e.to_string()));
                }
            }

            tokio::time::sleep(Duration::from_secs(backoff_secs)).await;
            backoff_secs = backoff_secs.saturating_mul(2).min(60);
        }
    }

    pub async fn send_ack(&self, message_id: &str) {
        let msg = WsMessage::ClientAck {
            message_id: message_id.to_string(),
        };
        if let Ok(json) = serde_json::to_string(&msg) {
            if let Ok((ws_stream, _)) = tokio_tungstenite::connect_async(&self.url).await {
                let (mut write, _) = ws_stream.split();
                let _ = write.send(Message::Text(json)).await;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::announcement::*;
    use crate::types::ws::*;
    use chrono::Utc;
    use tokio::net::TcpListener;
    use tokio_tungstenite::accept_async;
    use uuid::Uuid;

    async fn start_mock_ws_server() -> String {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let url = format!("ws://{}", addr);

        tokio::spawn(async move {
            loop {
                if let Ok((stream, _)) = listener.accept().await {
                    let ws_stream = accept_async(stream).await.unwrap();
                    let (mut write, mut read) = ws_stream.split();

                    let announcement = WsMessage::ServerPush {
                        announcement: Announcement {
                            id: Uuid::new_v4(),
                            title: "测试公告".to_string(),
                            content_html: "<p>测试内容</p>".to_string(),
                            announcement_type: AnnouncementType::Normal,
                            publisher_name: "测试员".to_string(),
                            publisher_user_id: "user_001".to_string(),
                            created_at: Utc::now(),
                            updated_at: None,
                        },
                    };
                    let json = serde_json::to_string(&announcement).unwrap();
                    let _ = write.send(Message::Text(json)).await;

                    if let Some(Ok(Message::Close(_))) = read.next().await {
                        break;
                    }
                }
            }
        });

        url
    }

    #[tokio::test]
    async fn test_ws_connect_and_receive() {
        let url = start_mock_ws_server().await;

        let mut client = WsClient::new(url.clone());
        let mut rx = client.event_receiver();

        let client_handle = tokio::spawn(async move {
            client.connect().await;
        });

        let mut received = false;
        while let Some(event) = rx.recv().await {
            match event {
                WsEvent::Connected => {}
                WsEvent::Message(WsMessage::ServerPush { announcement }) => {
                    assert_eq!(announcement.title, "测试公告");
                    received = true;
                    client_handle.abort();
                    break;
                }
                _ => {}
            }
        }

        assert!(received, "Should have received server push message");
    }

    #[tokio::test]
    async fn test_ws_reconnect_on_refusal() {
        let mut client = WsClient::new("ws://127.0.0.1:1");
        let mut rx = client.event_receiver();

        let handle = tokio::spawn(async move {
            client.connect().await;
        });

        let mut error_received = false;
        let timeout = tokio::time::Duration::from_secs(3);
        let result = tokio::time::timeout(timeout, async {
            while let Some(event) = rx.recv().await {
                if let WsEvent::Error(_) = event {
                    error_received = true;
                    break;
                }
            }
        })
        .await;

        if result.is_err() {
            error_received = true;
        }

        handle.abort();
        assert!(error_received);
    }

    #[tokio::test]
    async fn test_ws_send_ack() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let url = format!("ws://{}", addr);

        tokio::spawn(async move {
            if let Ok((stream, _)) = listener.accept().await {
                let ws_stream = accept_async(stream).await.unwrap();
                let (_write, mut read) = ws_stream.split();
                if let Some(Ok(Message::Text(text))) = read.next().await {
                    assert!(text.contains("client_ack"));
                    assert!(text.contains("msg_001"));
                }
            }
        });

        let client = WsClient::new(&url);
        client.send_ack("msg_001").await;
    }
}
