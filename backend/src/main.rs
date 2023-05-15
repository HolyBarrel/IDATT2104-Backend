use std::thread;
use std::sync::{Arc, Mutex};
use tungstenite::{WebSocket, Message, accept};
use std::net::{TcpListener,TcpStream};
mod structures;
use structures::node::Node;


fn main() {
    let server = TcpListener::bind("10.24.36.138:8765").unwrap();
    let connections = Arc::new(Mutex::new(Vec::new()));

    for stream in server.incoming() {
        let connections = connections.clone();
        let stream = stream.unwrap();

        let handle = thread::spawn(move || {
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
            let text = "Hello, front-end!";
            socket.write_message(Message::Text(text.to_owned())).unwrap();
        }
    }
}
