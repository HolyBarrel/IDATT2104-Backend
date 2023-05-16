use std::thread;
use std::sync::{Arc, Mutex};
use serde::de::Error;
use serde_json::{Result, Value, from_str, to_string};
use tungstenite::{WebSocket, Message, accept};
use std::net::{TcpListener,TcpStream};
mod structures;
use structures::dto::NodeDTO;
use structures::node::Node;
use structures::node_queue::NodeQueue;
use std::collections::HashSet;

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
            let node = match parse_json(msg) {
                Ok(node) => node,
                Err(e) => {
                    println!("Failed to parse message: {:?}", e);
                    continue;
                }
            };
            let text = to_string(&node).unwrap();
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

//Takes in a node and adds all of its neighbors to the queue
fn spread_signal(node: Node, queue: &mut NodeQueue) {
    /*
    let mut visited: HashSet<Node> = HashSet::new();
    let neighbor_positions = node.adj_positions();
    for position in neighbor_positions {
        let neighbor = Node::new(position.0, position.1);
        if !visited.contains(&neighbor) {
            queue.add(neighbor);
            visited.insert(neighbor);
        }
    }
    */
}