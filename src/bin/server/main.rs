use crate::chat_maps::ChatTracker;
use crate::connection::handle;
use async_std::net;
use chat::utils::ChatResult;
use futures_lite::StreamExt;
use std::sync::Arc;

mod chat_maps;
mod chats;
mod connection;

fn main() -> ChatResult<()> {
    let addr = std::env::args().nth(1).expect("server ADDRESS");
    let chats_map = Arc::new(ChatTracker::new());

    async_std::task::block_on(async {
        let listener = net::TcpListener::bind(addr).await?;

        while let Some(res) = listener.incoming().next().await {
            let stream = res?;
            let chats = chats_map.clone();
            async_std::task::spawn(async { log_error(handle(stream, chats).await) });
        }

        Ok(())
    })
}

fn log_error(res: ChatResult<()>) {
    if let Err(e) = res {
        println!("An error occurred: {}", e)
    }
}
