use std::thread;
use std::sync::{Arc, Mutex};
use serde::de::Error;
use serde_json::{Result, Value, from_str, to_string};
use tungstenite::{WebSocket, Message, accept};
use std::net::{TcpListener,TcpStream};
mod structures;
use structures::node::Node;
use structures::dto::NodeDTO;
use structures::dto::answerDTO;

fn main() {
    let server = TcpListener::bind("127.0.0.1:8765").unwrap();
    let connections = Arc::new(Mutex::new(Vec::new()));

    for stream in server.incoming() {
        let connections = connections.clone();
        let stream = stream.unwrap();
        thread::spawn(move  ||{
            let mut socket = accept(stream).unwrap();
            connections.lock().unwrap().push(socket.get_ref().peer_addr().unwrap());
            handle_connection(&mut socket);
            // Remove the connection from the list when the thread is finished
            let mut connections = connections.lock().unwrap();
            connections.retain(|c| *c != socket.get_ref().peer_addr().unwrap());
        });
    }
}

fn handle_connection(socket: &mut WebSocket<TcpStream>) {
    while let Ok(msg) = socket.read_message() {
        if msg.is_binary() || msg.is_text() {
            println!("Received message: {:?}", msg);
            let nodes = match parse_json(msg) {
                Ok(nodes) => nodes,
                Err(e) => {
                    println!("Failed to parse message: {:?}", e);
                    continue;
                }
            };
            let mut answer = vec![];
            for a in nodes {
                answer.push(answerDTO::new(*a.get_x(),*a.get_y()));
            }
            let text = to_string(&answer).unwrap();
            socket.write_message(Message::Text(text)).unwrap();
        }
    }
}

fn parse_json(msg: Message) -> Result<Vec<NodeDTO>> {
    match msg {
        Message::Text(text) => {
            let dto: Vec<NodeDTO> = from_str(&text)?;
            Ok(dto)
        },
        _ => {
            Err(Error::custom("Not json parsable!"))
        }
    }
}