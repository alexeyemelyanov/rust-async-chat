use std::sync::Arc;
use serde::{Deserialize, Serialize};

pub mod utils;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum ClientAction {
    Join {
        chat_name: Arc<String>
    },
    Post {
        chat_name: Arc<String>,
        message: Arc<String>
    }
}

#[derive(Deserialize, Serialize)]
pub enum ServerEvent {
    Message {
        chat_name: Arc<String>,
        message: Arc<String>
    },
    Error(String)
}