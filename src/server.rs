use std::{net::SocketAddr, collections::HashMap, sync::Arc};
use tokio::{net::TcpListener, io::{AsyncWriteExt, BufReader, AsyncBufReadExt, AsyncReadExt}, sync::{broadcast::{channel, error::RecvError}, Mutex}};
use crate::{message::{Message, Command, COMMAND_PREFIX}, user::User};

#[tokio::main]
pub async fn run(port: u16, size: usize) {
    let listener = TcpListener::bind(("localhost", port)).await.unwrap();
    println!("opened chat");

    let (sender, mut receiver) = channel::<(Message, SocketAddr)>(size);
    let mut users = Arc::new(Mutex::new(HashMap::<SocketAddr, User>::new()));

    loop {
        let mut users = Arc::clone(&users);
        let Ok((mut socket, addr)) = listener.accept().await else {
            continue;
        };
        users.lock().await.insert(addr, User::default());
        println!("accpected {addr:?}");

        let sender = sender.clone();
        let mut receiver = sender.subscribe();

        tokio::spawn(async move {
            let (reader, mut writer) = socket.split();
            let mut reader = BufReader::new(reader);
            
            'session: loop {
                let mut text = String::new();
                tokio::select! {
                    result = reader.read_line(&mut text) => match result {
                        Ok(len) => {
                            if let Some(user) = users.lock().await.get_mut(&addr) {
                                if len == 0 {
                                    break 'session;
                                }
                                let msg = Message::new(text);
                                println!("{addr:?} ({:?}) send: {msg:?}", user.name);
                                if msg.text.starts_with(COMMAND_PREFIX) {
                                    match Command::try_from(msg) {
                                        Ok(command) => match command {
                                            Command::Nickname(name) => {
                                                println!("changed nickname to {name:?}");
                                                writer.write_all(format!("changed nickname to {name:?}\n").as_bytes()).await;
                                                user.name = Some(name);
                                            }
                                            Command::Exit => {
                                                println!("{addr:?} ({:?}) exited", user.name);
                                                break 'session;
                                            }
                                        }
                                        Err(parse_error) => {
                                            println!("parse error: {parse_error}");
                                            let mut text = parse_error.to_string();
                                            text.push('\n');
                                            writer.write_all(text.as_bytes()).await;
                                        }
                                    }
                                } else {
                                    sender.send((msg, addr));
                                }
                            }
                        }
                        Err(err) => {
                            let user = users.lock().await.remove(&addr);
                            println!("removed {addr:?} ({:?}) due to: {err}", user.and_then(|user| user.name));
                            break 'session;
                        }
                    },
                    result = receiver.recv() => match result {
                        Ok((msg, other_addr)) => {
                            if addr != other_addr {
                                let mut text = String::new();
                                if let Some(user) = users.lock().await.get(&other_addr) {
                                    text.push('[');
                                    text.push_str(&user.name.clone().unwrap_or_else(|| addr.ip().to_string()));
                                    text.push(']');
                                    text.push(' ');
                                }
                                text.push_str(&msg.text);
                                writer.write_all(text.as_bytes()).await;
                            }
                        }
                        Err(err) => {
                            let user = users.lock().await.remove(&addr);
                            println!("removed {addr:?} ({:?}) due to: {err}", user.and_then(|user| user.name));
                            break 'session;
                        }
                    }
                }
            }
        });
    }
}