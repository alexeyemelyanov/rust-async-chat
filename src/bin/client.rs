use async_std::io::BufReadExt;
use async_std::net::TcpStream;
use async_std::{io, task};
use chat::utils::{receive, send_json, ChatResult};
use chat::{ClientAction, ServerEvent};
use futures_lite::future::FutureExt;
use futures_lite::StreamExt;
use std::sync::Arc;

fn get_value(input: &str) -> Option<(&str, &str)> {
    let input = input.trim_start();
    if input.is_empty() {
        None
    } else {
        match input.find(char::is_whitespace) {
            None => Some((input, "")),
            Some(whitespace) => Some((&input[0..whitespace], &input[whitespace..])),
        }
    }
}

fn parse_input(line: &str) -> Option<ClientAction> {
    let (input, reminder) = get_value(line)?;

    match input {
        "join" => {
            let (chat, reminder) = get_value(reminder)?;
            if !reminder.trim_start().is_empty() {
                None
            } else {
                Some(ClientAction::Join {
                    chat_name: Arc::new(String::from(chat)),
                })
            }
        }
        "post" => {
            let (chat, reminder) = get_value(reminder)?;
            let message = reminder.trim_start();
            if message.is_empty() {
                None
            } else {
                Some(ClientAction::Post {
                    chat_name: Arc::new(String::from(chat)),
                    message: Arc::new(String::from(message)),
                })
            }
        }
        _ => {
            println!("Unrecognized input: {}", line);
            None
        }
    }
}

async fn send(mut send: TcpStream) -> ChatResult<()> {
    println!("Options:\njoin CHAT\npost CHAT MESSAGE");

    let mut options = io::BufReader::new(io::stdin()).lines();
    while let Some(input) = options.next().await {
        let option = match parse_input(&input?) {
            None => continue,
            Some(req) => req,
        };
        send_json(&mut send, &option).await?;
    }

    Ok(())
}

async fn messages(server: TcpStream) -> ChatResult<()> {
    let reader = io::BufReader::new(server);
    let mut stream = receive(reader);

    while let Some(msg) = stream.next().await {
        match msg? {
            ServerEvent::Error(err) => {
                println!("Error received: {}", err)
            }
            ServerEvent::Message { chat_name, message } => {
                println!("Chat: {}\nMessage: {}", chat_name, message)
            }
        }
    }

    Ok(())
}

fn main() -> ChatResult<()> {
    let addr = std::env::args().nth(1).expect("Address:PORT");

    task::block_on(async {
        let stream = TcpStream::connect(addr).await?;
        stream.set_nodelay(true)?;

        let send = send(stream.clone());
        let replies = messages(stream);

        replies.race(send).await?;

        Ok(())
    })
}
