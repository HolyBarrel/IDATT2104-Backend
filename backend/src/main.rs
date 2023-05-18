use std::thread;
use std::sync::{Arc, Mutex, RwLock, RwLockReadGuard};
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
use queues::*;

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
    let antennas = Arc::new(RwLock::new(vec_buildings.clone()));
    let extenders= Arc::new(RwLock::new(vec_buildings));
    let board_lock = Arc::new(RwLock::new(vec![vec![Node::new(-1, -1);100];100]));
    while let Ok(msg) = socket.read_message() {
        let network_type_clone= network_type.clone();
        if msg.is_binary() || msg.is_text() {

            let mut msgClone = msg.clone();
            let textMsg = msgClone.to_text().unwrap();
            //println!("{:?}", msgClone);
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
            for node in nodes {
                node_clone.push(node.clone());
                match node.get_building(){
                    Some(value) =>{
                        if(value.eq_ignore_ascii_case("antennas")){
                            let mut guard = antennas.write().unwrap();
                            guard.push(Building::new(*node.get_x(), *node.get_y(), value));
                        }else {
                            let mut guard = extenders.write().unwrap();
                            guard.push(Building::new(*node.get_x(), *node.get_y(), value));
                        }
                        
                    } 
                    None =>{
                        let mut guard = antennas.write().unwrap();
                        let mut copy = guard.clone();
                        remove_building(*node.get_x(), *node.get_y(), &mut copy);
                        guard.clear();
                        guard.extend(copy);
                    
                    } 
                }
          
                
            }

            clean_board(&board_lock);
            let mut guard = board_lock.read().unwrap();
            let clean_board = convert_board_to_dto(&mut guard);
            socket.write_message(Message::Text(to_string(&clean_board).unwrap())).unwrap();
            let queue = NodeQueue::new_queue();
            {
                let mut guard = board_lock.write().unwrap();
                populate_board(&mut guard, node_clone);
                let building_guard = antennas.read().unwrap();
                for x in building_guard.iter(){
                    spread_signal(guard[*x.get_x() as usize][*x.get_y() as usize].clone(),&mut guard, socket);
                }

                //TODO: iterate thorugh buildings and spread signal from each building before spreading signal from the extenders
                //TODO2: update all signals in the board, remember the ones outside the area changed

            }
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

fn convert_board_to_dto(board: &mut RwLockReadGuard<Vec<Vec<Node>>>) -> Vec<answerDTO> {
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
        node.set_building(building.to_string());
        node.set_weight();
        board[x][y] = node;
    }
}

//Takes in a node and adds all of its neighbours to the queue
fn spread_signal(mut node: Node, board: &mut Vec<Vec<Node>>, socket: &mut WebSocket<TcpStream>) {
    let mut node_queue = NodeQueue::new_queue();
    let mut signal_queue = Queue::<i32>::new();
    let mut visited = HashSet::<Node>::new();


    print!("This is the building: {:?}", node.get_building());
    /*
    if node.get_building() == "tower" {
        node.set_output(100);
    }

    else if node.get_building() == "extender" {
        node.set_output(node.get_output() + 50);
    }
    */

    //TODO: implement circular signal spreading
    //TODO: 5g, 4g, 3g, 2g, 1g

    node.set_output(100);

    let mut mountain_source = false;
    if(node.get_landscape() == "mountain"){
        mountain_source = true;
    }

    //Add the first node to the board
    board[*node.get_x() as usize][*node.get_y() as usize] = node.clone();
    let dto = node.convert_to_DTO();
    let temp = vec![dto];
    socket.write_message(Message::Text(to_string(&temp).unwrap())).unwrap();

    node_queue.add(node.clone());
    signal_queue.add(*node.get_output());
    visited.insert(node.clone());

    let mut iteration_count = 0;
    while node_queue.size() > 0 && iteration_count < 100000 {
        //if iteration_count >= 100000 {
            //println!("Max iteration limit reached");
        //}

        if let Ok(current_node) = node_queue.pop_first() {
            let current_signal = signal_queue.remove().unwrap();
            let neighbour_positions = current_node.adj_positions();

            for position in neighbour_positions {
                let x = position.0;
                let y = position.1;

                if x >= 0 && x < 100 && y >= 0 && y < 100 {
                    let x_usize = x as usize;
                    let y_usize = y as usize;
                    let mut neighbour = board[x_usize][y_usize].clone();
                    let mut clone_neighbour = neighbour.clone();

                    if (current_signal > 0 && neighbour.get_output() < &current_signal) && (!visited.contains(&neighbour) || (neighbour.get_output() > &0)) {
                        neighbour.set_input(current_signal, mountain_source);
                        let output_signal = neighbour.get_output();

                        if output_signal > &0 {
                            visited.insert(neighbour.clone());
                            node_queue.add(neighbour.clone());
                            signal_queue.add(*output_signal);

                            board[x_usize][y_usize] = neighbour;

                        }
                    }
                    let dto = clone_neighbour.convert_to_DTO();
                    let temp = vec![dto];
                    socket.write_message(Message::Text(to_string(&temp).unwrap())).unwrap();
                }

            }

            iteration_count += 1;
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
            column.set_input(0, false);
        }
    }
}
