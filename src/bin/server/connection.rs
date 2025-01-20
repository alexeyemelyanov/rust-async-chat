use async_std::io;
use async_std::net::TcpStream;
use async_std::sync::Mutex;
use chat::utils::{receive, send_json, ChatResult};
use chat::{ClientAction, ServerEvent};
use futures_lite::StreamExt;
use std::sync::Arc;

use crate::chat_maps::ChatTracker;

pub struct Leaving(Mutex<TcpStream>);

impl Leaving {
    fn new(client: TcpStream) -> Leaving {
        Leaving(Mutex::new(client))
    }

    pub async fn send(&self, event: ServerEvent) -> ChatResult<()> {
        let mut lock = self.0.lock().await;
        send_json(&mut *lock, &event).await
    }
}

pub async fn handle(tcp_stream: TcpStream, chats: Arc<ChatTracker>) -> ChatResult<()> {
    let leaving = Arc::new(Leaving::new(tcp_stream.clone()));
    let reader = io::BufReader::new(tcp_stream);
    let mut stream = receive(reader);

    while let Some(msg) = stream.next().await {
        let result = match msg? {
            ClientAction::Join { chat_name } => {
                let chat = chats.find_or_new(chat_name).await;
                chat.join(leaving.clone());
                Ok(())
            }
            ClientAction::Post { chat_name, message } => match chats.find(&chat_name).await {
                None => Err(format!("chat {} was not found", chat_name)),
                Some(chat) => {
                    chat.post(message);
                    Ok(())
                }
            },
        };

        if let Err(error) = result {
            let report = ServerEvent::Error(error);
            leaving.send(report).await?
        }
    }

    Ok(())
}
