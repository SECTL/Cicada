use serde::{Deserialize, Serialize};

use super::announcement::Announcement;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum WsMessage {
    ServerPush { announcement: Announcement },
    ClientAck { message_id: String },
    Heartbeat { timestamp: i64 },
    Error { code: String, message: String },
}
