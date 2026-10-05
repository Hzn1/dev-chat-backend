use crate::models::ChatMessage;
use std::sync::Arc;
use tokio::sync::Mutex;

pub type CHAT_DB = Arc<Mutex<Vec<ChatMessage>>>;

pub fn chat_db() -> CHAT_DB {
    Arc::new(Mutex::new(Vec::new()))
}
