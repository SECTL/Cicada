use std::sync::Arc;

use axum::extract::ws::{Message, WebSocket};
use axum::extract::{Path, Query, State, WebSocketUpgrade};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};
use chrono::Utc;
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use tokio::sync::{broadcast, RwLock};
use tower_http::cors::{Any, CorsLayer};
use uuid::Uuid;

use cicada_core::types::announcement::{
    Announcement, AnnouncementCreate, AnnouncementType,
};

type AnnouncementsDb = Arc<RwLock<Vec<Announcement>>>;
type BroadcastTx = broadcast::Sender<Announcement>;

#[derive(Clone)]
struct AppState {
    db: AnnouncementsDb,
    tx: BroadcastTx,
}

#[derive(Deserialize)]
struct ListQuery {
    page: Option<u32>,
    per_page: Option<u32>,
    #[serde(rename = "type")]
    announcement_type: Option<AnnouncementType>,
}

#[derive(Serialize)]
struct ListResponse {
    announcements: Vec<Announcement>,
    total: u64,
    page: u32,
    per_page: u32,
}

async fn seed_data(db: &AnnouncementsDb) {
    let mut announcements = db.write().await;
    let now = Utc::now();

    announcements.push(Announcement {
        id: Uuid::new_v4(),
        title: "欢迎使用知了校园公告系统".to_string(),
        content_html: "<p style='font-size:20px'>知了已就绪，等待公告推送...</p>".to_string(),
        announcement_type: AnnouncementType::Normal,
        publisher_name: "系统".to_string(),
        publisher_user_id: "system".to_string(),
        created_at: now,
        updated_at: None,
    });

    announcements.push(Announcement {
        id: Uuid::new_v4(),
        title: "关于期末考试安排的通知".to_string(),
        content_html: "<p>各位同学：<br/>本学期期末考试将于<b>6月20日</b>开始，请提前做好准备。</p>".to_string(),
        announcement_type: AnnouncementType::Notice,
        publisher_name: "教务处".to_string(),
        publisher_user_id: "user_admin".to_string(),
        created_at: now,
        updated_at: None,
    });

    announcements.push(Announcement {
        id: Uuid::new_v4(),
        title: "⚠️ 紧急通知：周三停课".to_string(),
        content_html: "<p style='color:red;font-size:24px'><b>由于暴雪天气，本周三（6月18日）全校停课一天。</b></p><p>请各位师生注意安全，尽量减少外出。</p>".to_string(),
        announcement_type: AnnouncementType::Emergency,
        publisher_name: "校长办公室".to_string(),
        publisher_user_id: "user_principal".to_string(),
        created_at: now,
        updated_at: None,
    });
}

async fn list_announcements(
    Query(query): Query<ListQuery>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let announcements = state.db.read().await;
    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(20).min(100);

    let filtered: Vec<Announcement> = if let Some(ref t) = query.announcement_type {
        announcements
            .iter()
            .filter(|a| a.announcement_type == *t)
            .cloned()
            .collect()
    } else {
        announcements.clone()
    };

    let total = filtered.len() as u64;
    let start = ((page - 1) * per_page) as usize;
    let paged: Vec<Announcement> = filtered.into_iter().skip(start).take(per_page as usize).collect();

    Json(ListResponse {
        announcements: paged,
        total,
        page,
        per_page,
    })
}

async fn get_announcement(
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
) -> Result<Json<Announcement>, StatusCode> {
    let announcements = state.db.read().await;
    announcements
        .iter()
        .find(|a| a.id == id)
        .cloned()
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

async fn create_announcement(
    State(state): State<AppState>,
    Json(payload): Json<AnnouncementCreate>,
) -> Result<(StatusCode, Json<Announcement>), StatusCode> {
    let now = Utc::now();
    let announcement = Announcement {
        id: Uuid::new_v4(),
        title: payload.title,
        content_html: payload.content_html,
        announcement_type: payload.announcement_type,
        publisher_name: payload.publisher_name,
        publisher_user_id: "mock_user".to_string(),
        created_at: now,
        updated_at: None,
    };

    state.db.write().await.push(announcement.clone());
    let _ = state.tx.send(announcement.clone());

    Ok((StatusCode::CREATED, Json(announcement)))
}

async fn delete_announcement(
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
) -> StatusCode {
    let mut announcements = state.db.write().await;
    if let Some(pos) = announcements.iter().position(|a| a.id == id) {
        announcements.remove(pos);
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_ws(socket, state.db))
}

async fn handle_ws(socket: WebSocket, db: AnnouncementsDb) {
    let (mut sender, mut receiver) = socket.split();

    {
        let announcements = db.read().await;
        for ann in announcements.iter() {
            let msg = serde_json::json!({
                "type": "server_push",
                "announcement": ann
            });
            if sender.send(Message::Text(msg.to_string())).await.is_err() {
                return;
            }
        }
    }

    let mut heartbeat = tokio::time::interval(tokio::time::Duration::from_secs(30));

    loop {
        tokio::select! {
            _ = heartbeat.tick() => {
                if sender.send(Message::Ping(vec![])).await.is_err() {
                    break;
                }
            }
            msg = receiver.next() => {
                match msg {
                    Some(Ok(Message::Close(_))) | None => break,
                    Some(Ok(Message::Pong(_))) => {}
                    _ => {}
                }
            }
        }
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let db: AnnouncementsDb = Arc::new(RwLock::new(Vec::new()));
    seed_data(&db).await;

    let (tx, _) = broadcast::channel::<Announcement>(256);

    let state = AppState {
        db: db.clone(),
        tx: tx.clone(),
    };

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/api/v1/announcements", get(list_announcements).post(create_announcement))
        .route("/api/v1/announcements/{id}", get(get_announcement).delete(delete_announcement))
        .route("/ws/v1/announcements", get(ws_handler))
        .layer(cors)
        .with_state(state);

    let addr = "0.0.0.0:3001";
    println!("Cicada Mock Server running at http://{}", addr);
    println!("  REST API: http://{}/api/v1/announcements", addr);
    println!("  WebSocket: ws://{}/ws/v1/announcements", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
