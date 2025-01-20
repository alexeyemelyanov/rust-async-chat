use crate::chats::Chat;
use async_std::sync::Mutex;
use std::collections::HashMap;
use std::sync::Arc;

pub struct ChatTracker(Mutex<HashMap<Arc<String>, Arc<Chat>>>);

impl ChatTracker {
    pub fn new() -> ChatTracker {
        ChatTracker(Mutex::new(HashMap::new()))
    }
    pub async fn find_or_new(&self, chat_name: Arc<String>) -> Arc<Chat> {
        self.0
            .lock()
            .await
            .entry(chat_name.clone())
            .or_insert_with(|| Arc::new(Chat::new(chat_name)))
            .clone()
    }
    pub async fn find(&self, chat_name: &String) -> Option<Arc<Chat>> {
        self.0.lock().await.get(chat_name).cloned()
    }
}
