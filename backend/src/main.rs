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
use std::sync::RwLockWriteGuard;

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
    let network_type = Arc::new(RwLock::new("5G".to_string()));
    let vec_buildings:Vec<Building> = vec![];
    let buildings = Arc::new(RwLock::new(vec_buildings));
    let board_lock = Arc::new(RwLock::new(vec![vec![Node::new(-1, -1);100];100]));
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
            let mut clean = false;
            //let mut answer = vec![];
            for node in nodes {
                node_clone.push(node.clone());
                match node.get_building(){
                    Some(value) =>{
                        //println!("{:?}",value);
                        let mut guard = buildings.write().unwrap();
                        guard.push(Building::new(*node.get_x(), *node.get_y(), value));
                    } 
                    None =>{
                        let mut guard = buildings.write().unwrap();
                        let mut copy = guard.clone();
                        remove_building(*node.get_x(), *node.get_y(), &mut copy);
                        guard.clear();
                        guard.extend(copy);
                        clean = true;
                    } 
                }

                if clean{
                    clean_board(&board_lock);
                }
                //let mut guard = board_lock.write().unwrap();
                //guard[*node.get_x() as usize][*node.get_y() as usize] = node.get_node();
            }

            

            let queue = NodeQueue::new_queue();
            {
                let mut guard = board_lock.write().unwrap();
                populate_board(&mut guard, node_clone);
                let building_guard = buildings.read().unwrap();
                for x in building_guard.iter(){
                    spread_signal(guard[*x.get_x() as usize][*x.get_y() as usize].clone(),&mut guard);
                }
                
            }
            
            let mut guard = board_lock.write().unwrap();
            let answerVec = convert_board_to_dto(&mut guard);
            let text = to_string(&answerVec).unwrap();
            //println!("Board = {:?}",board_lock);
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

fn convert_board_to_dto(board: &mut RwLockWriteGuard<Vec<Vec<Node>>>) -> Vec<answerDTO> {
    let mut nodes_dto = Vec::new();

    for row in board.iter() {
        for node in row.iter() {
            let answerDTO = node.convert_to_DTO();
            nodes_dto.push(answerDTO);
        }
    }

    nodes_dto
}

fn populate_board(board: &mut RwLockWriteGuard<Vec<Vec<Node>>>, nodes: Vec<NodeDTO>) {
    for node in nodes {
        let x = *node.get_x() as usize;
        let y = *node.get_y() as usize;
        let landscape = node.get_landscape().unwrap_or("field".to_string());
        let building = node.get_building().unwrap_or("none".to_string());
        let mut node = Node::new_from_usize(x, y);
        node.set_landscape(landscape.to_string());
        node.set_building(building.to_owned());
        node.set_weight();
        board[x][y] = node;
    }
}

// Takes in a node and adds all of its neighbours to the queue
fn spread_signal(node: Node, board: &mut Vec<Vec<Node>>) {
    let mut queue = NodeQueue::new_queue();
    let mut neighbour_positions = node.adj_positions();
    for position in neighbour_positions {
        let x = position.0;
        let y = position.1;
        if x >= 0 && x < 100 && y >= 0 && y < 100 {
            let x_usize = x as usize;
            let y_usize = y as usize;
            let signal_strength = 100;
            let mut neighbour = board[x_usize][y_usize].clone();
            let int = neighbour.set_input(signal_strength);
            println!("{:?}", int);
            queue.add(neighbour);
        }
    }
    //println!("{:?}", queue);
    while queue.size() > 0 {
        let node = queue.pop_first();
        match node {
            Ok(value) => {
                let x = value.clone();
                board[*x.get_x() as usize][*x.get_y() as usize] = value;
            }
            Err(_) => {}
        }
    }
}

fn remove_building(x: i32, y: i32, buildings: &mut Vec<Building>) {
    buildings.retain(|building| building.get_x() != &x || building.get_y() != &y);
}

fn clean_board(board: &Arc<RwLock<Vec<Vec<Node>>>>) {
    let mut guard = board.write().unwrap();
    for row in guard.iter_mut() {
        for column in row.iter_mut() {
            column.set_input(0);
        }
    }
}
