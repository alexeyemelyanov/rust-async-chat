use crate::connection::Leaving;
use async_std::task;
use chat::ServerEvent;
use std::sync::Arc;
use tokio::sync::broadcast;
use tokio::sync::broadcast::error::RecvError;
use tokio::sync::broadcast::Receiver;

pub struct Chat {
    name: Arc<String>,
    publisher: broadcast::Sender<Arc<String>>,
}

impl Chat {
    pub fn new(name: Arc<String>) -> Chat {
        let (publisher, _) = broadcast::channel(1000);
        Chat { name, publisher }
    }
    pub fn post(&self, message: Arc<String>) {
        let _ = self.publisher.send(message);
    }
    pub fn join(&self, leaving: Arc<Leaving>) {
        let receiver = self.publisher.subscribe();
        task::spawn(sub(self.name.clone(), receiver, leaving));
    }
}

async fn sub(chat_name: Arc<String>, mut receiver: Receiver<Arc<String>>, leaving: Arc<Leaving>) {
    loop {
        let packet = match receiver.recv().await {
            Ok(message) => ServerEvent::Message {
                chat_name: chat_name.clone(),
                message: message.clone(),
            },
            Err(RecvError::Lagged(n)) => {
                ServerEvent::Error(format!("Dropped {} messages from {}.", n, chat_name))
            }
            Err(RecvError::Closed) => break,
        };

        if leaving.send(packet).await.is_err() {
            break;
        }
    }
}
