use std::thread;
use std::sync::{Arc, Mutex, RwLock};
use serde::de::Error;
use serde_json::{Result, Value, from_str, to_string};
use tungstenite::{WebSocket, Message, accept};
use std::net::{TcpListener,TcpStream};
mod structures;
use structures::dto::NodeDTO;
use structures::node::Node;
use structures::node_queue::NodeQueue;
use std::collections::HashSet;
use structures::dto::answerDTO;
use structures::building::Building;

fn main() {
    let server = TcpListener::bind("192.168.0.188:8765").unwrap();
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
    let network_type = Arc::new(RwLock::new("5G".to_string()));
    let vec_buildings:Vec<Building> = vec![];
    let buildings = Arc::new(RwLock::new(vec_buildings));
    let mut board: Vec<Vec<Node>> = vec![];
    for i in 0..100{
        let mut temp:Vec<Node> = vec![Node::new(-1, -1);100];
        board.push(temp);
    }
    while let Ok(msg) = socket.read_message() {
        let network_type_clone= network_type.clone();
        if msg.is_binary() || msg.is_text() {

            let mut msgClone = msg.clone();
            let textMsg = msgClone.to_text().unwrap();
            println!("{:?}", msgClone);
            if(textMsg.starts_with("?")){
                let subText = &textMsg[1..];
                let mut guard = network_type_clone.write().unwrap();
                *guard = subText.to_string();
            }



            let mut nodes = match parse_json(msg) {
                Ok(nodes) => nodes,
                Err(e) => {
                    continue;
                }
            };
            
            let mut node_clone = vec![];

            //TODO: FIX BUILDING NAME 
            //let mut answer = vec![];
            for a in nodes {
                node_clone.push(a.clone());
                match a.get_building(){
                    Some(value) =>{
                        let mut guard = buildings.write().unwrap();
                        guard.push(Building::new(*a.get_x(), *a.get_y(), value));
                    } 
                    None =>{
                    } 
                }
            }
            let mut queue = NodeQueue::new_queue();
            populate_board(&mut board, node_clone);
            println!("{:?}", buildings);
            //spread_signal(board[10][10].clone(), &mut queue, &mut board);
            println!("Queue = {:?}", queue);

            let text = to_string("Hi").unwrap();
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

fn populate_board(board: &mut Vec<Vec<Node>>, nodes: Vec<NodeDTO>) {
    for node in nodes {
        let x = *node.get_x() as usize;
        let y = *node.get_y() as usize;
        let landscape = node.get_landscape().unwrap_or("field".to_string());
        let building = node.get_building().unwrap_or("none".to_string());
        let mut node = Node::new_from_usize(x, y);
        node.set_landscape(landscape.to_string());
        node.set_building(building.to_owned());
        node.set_weight();
        board[x][y] = node
    }
}

//Takes in a node and adds all of its neighbors to the queue
fn spread_signal(node: Node, queue: &mut NodeQueue, board: &mut Vec<Vec<Node>>) {
    let mut neighbour_positions = node.adj_positions();
    for position in neighbour_positions {
        let x = position.0;
        let y = position.1;
        if x >= 0 && x < 100 && y >= 0 && y < 100 {
            let x_usize = x as usize;
            let y_usize = y as usize;
            let signal_strength = 100;
            let mut neighbour = board[x_usize][y_usize].clone();
            neighbour.set_input(signal_strength);
            queue.add(neighbour);
        }
    }
}

