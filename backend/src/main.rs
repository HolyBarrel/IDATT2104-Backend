use std::thread;
use std::sync::{Arc, Mutex, RwLock};
use serde::de::Error;
use serde_json::{Result, from_str, to_string};
use tungstenite::{WebSocket, Message, accept};
use std::net::{TcpListener,TcpStream};
mod structures;
use structures::{
    dto::{NodeDTO, AnswerDTO},
    node::Node,
    node_queue::NodeQueue,
    building::Building,
};
use std::collections::HashSet;
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
            //Remove the connection from the list when the thread is finished
            let mut connections = connections.lock().unwrap();
            connections.retain(|c| *c != socket.get_ref().peer_addr().unwrap());
        });
    }
}

fn handle_connection(socket: &mut WebSocket<TcpStream>) {
    let network_type = Arc::new(RwLock::new("5G".to_string()));
    let vec_buildings:Vec<Building> = vec![];
    let vec_copy :Vec<Building>= vec![];
    let antennas = Arc::new(RwLock::new(vec_buildings));
    let extenders = Arc::new(RwLock::new(vec_copy));
    const BOARD_SIZE: usize = 100;
    let board_lock = Arc::new(RwLock::new(vec![vec![Node::new(-1, -1);BOARD_SIZE];BOARD_SIZE]));
    while let Ok(msg) = socket.read_message() {
        let network_type_clone= network_type.clone();
        
        // If the incoming message is binary or text
        if msg.is_binary() || msg.is_text() {

            // Cloning the message to be used later
            let msg_clone = msg.clone();
            let text_msg = msg_clone.to_text().unwrap();
           
            // Check if the message starts with '?'
            // If true, it changes the network type to the message content
            if text_msg.starts_with("?") {
                let sub_text = &text_msg[1..];
                let mut guard = network_type_clone.write().unwrap();
                *guard = sub_text.to_string();
            }
            // Parsing incoming JSON messages and if there is an error, it skips the rest of the loop iteration
            let nodes = match parse_json(msg) {
                Ok(nodes) => nodes,
                Err(_e) => {
                    continue;
                }
            };

            // Cloning all the nodes in a new Vector
            let mut node_clone = vec![];
            for node in nodes {
                node_clone.push(node.clone());

                // Checking if the node has a building 
                // If true, it adds the building to the antennas or extenders Vec, based on the building type
                // If false, it removes the building from the antennas and extenders Vec and clears them
                match node.get_building(){
                    Some(value) =>{
                        if value.starts_with("to") {
                            let mut guard = antennas.write().unwrap();
                            guard.push(Building::new(*node.get_x(), *node.get_y(), value));
                        }else if value.starts_with("ex"){
                            let mut guard = extenders.write().unwrap();
                            guard.push(Building::new(*node.get_x(), *node.get_y(), value));
                        }
                    } 
                    None =>{
                        let mut guard = antennas.write().unwrap();
                        let mut extender_gurd = extenders.write().unwrap();
                        let mut copy = guard.clone();
                        let extendet_copy = extender_gurd.clone();
                        remove_building(*node.get_x(), *node.get_y(), &mut copy);
                        remove_building(*node.get_x(), *node.get_y(), &mut extender_gurd);
                        guard.clear();
                        guard.extend(copy);
                        extender_gurd.clear();
                        extender_gurd.extend(extendet_copy);
                    
                    } 
                }

                // Creating a new node and setting its properties
                let x = *node.get_x() as usize;
                let y = *node.get_y() as usize;
                let landscape = node.get_landscape().unwrap_or("field".to_string());
                let building = node.get_building().unwrap_or("none".to_string());
                let mut node = Node::new_from_usize(x, y);
                node.set_landscape(landscape.to_string());
                node.set_building(building.to_string());
                node.set_weight();
                // Inserting the node to the board
                let mut guard = board_lock.write().unwrap();
                guard[x][y] = node;

            }

             // Sending back a JSON message to the client with a clean state of the board
            {
                let guard_network = network_type.read().unwrap();
                let guard_network_clone = guard_network.clone();
                clear_board(&board_lock, guard_network_clone.to_string());
                let mut guard = board_lock.write().unwrap();
                let mut answer = convert_board_to_dto(&mut guard);
                socket.write_message(Message::Text(to_string(&mut answer).unwrap())).unwrap();
            }

            // Checking the network type and updating the signal in the board according to the building type
            {   
                let read_guard_network = network_type.read().unwrap();
                let read_guard_network_clone = read_guard_network.clone();
    
                let mut guard = board_lock.write().unwrap();
                let building_guard = antennas.read().unwrap();
                
                // Loop through each building in the antennas vector
                for x in building_guard.iter(){
                    // Spread signal from the current antenna
                    spread_signal(guard[*x.get_x() as usize][*x.get_y() as usize].clone(),&mut guard, socket, read_guard_network_clone.to_string());
                }

                let mut building_guard: RwLockWriteGuard<Vec<Building>> = extenders.write().unwrap();
                let mut new_extenders= vec![];

                // Loop through each building in the extenders vector
                while !building_guard.is_empty() {
                    let building_clone = building_guard.clone();
                    let mut unique_buildings = HashSet::new();
                    
                    // Create a set of unique nodes for each building in the extenders
                    for building in building_clone.into_iter() {
                        let tem = guard[*building.get_x() as usize][*building.get_y() as usize].clone();
                        unique_buildings.insert(tem);
                    }
                    let mut sort_nodes: Vec<_> = unique_buildings.into_iter().collect();
                    sort_nodes.sort();
                    // Clear the building_guard and add the sorted nodes back
                    building_guard.clear();
                    for node in sort_nodes {
                        building_guard.push(Building::new(*node.get_x(), *node.get_y(), "extender".to_string()))
                    }

                     // Pop a node from the building_guard, spread its signal and add it to new_extenders
                    let x = building_guard.pop().unwrap();
                    spread_signal(guard[*x.get_x() as usize][*x.get_y() as usize].clone(),&mut guard, socket, read_guard_network_clone.to_string());
                    new_extenders.push(x);
                }
                for new in new_extenders{
                    building_guard.push(new);
                }
                
            }
        }
    }
}

// Parses a JSON message and returns a Vec<NodeDTO>
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

// Converts the board to a dto 
fn convert_board_to_dto(board: &mut RwLockWriteGuard<Vec<Vec<Node>>>) -> Vec<AnswerDTO> {
    let mut nodes_dto = Vec::new();

    for row in board.iter() {
        for node in row.iter() {
            let answer_dto = node.convert_to_dto();
            nodes_dto.push(answer_dto);
        }
    }

    nodes_dto
}

// Spreads the signal from the source node to its neighbours in a circular fashion
fn spread_signal(mut node: Node, board: &mut Vec<Vec<Node>>, socket: &mut WebSocket<TcpStream>, network_type: String) {

    // Creates the data structures needed for the algorithm
    let mut node_queue = NodeQueue::new_queue();
    let mut signal_queue = Queue::<i32>::new();
    let mut visited = HashSet::<(i32, i32)>::new();

    // Modified signal strength based on the network type
    let network_modifier = match network_type.as_str() {
        "4G" => 0.975,
        "3G" => 0.995,
        _ => 0.9,
    };

    // Checks if the node is a tower and sets its output to 100
    if node.get_building() == "tower" {
        node.set_output(100);
    }

    // Checks if the node is an extender and sets its output to 50 more than its received signal
    else if node.get_building() == "extender" {
        if node.get_output() > &0 {
            node.set_output(node.get_output() + 50);
        } else {
            node.set_output(0);
        }
    }

    // Checks if the node is placed on a mountain
    let mut mountain_source = false;
    if node.get_landscape() == "mountain" {
        mountain_source = true;
    }

    // Adds the first node to the board
    board[*node.get_x() as usize][*node.get_y() as usize] = node.clone();
    let dto = node.convert_to_dto();
    let temp = vec![dto];
    // Sends the first node to the client
    socket.write_message(Message::Text(to_string(&temp).unwrap())).unwrap();

    // Handles the first node
    node_queue.add(node.clone());
    signal_queue.add(*node.get_output()).unwrap();
    visited.insert((node.get_x().clone(), node.get_y().clone()));

    // Accounts for iteration count
    let mut iteration_count = 0;

    // Iterates through the node queue
    while node_queue.size() > 0 && iteration_count < 100000 {

        if let Ok(current_node) = node_queue.pop_first() {
            let current_signal = signal_queue.remove().unwrap();
            let neighbour_positions = current_node.adj_positions();

            // Iterates through the neighbours of the current node
            for position in neighbour_positions {
                let x = position.0;
                let y = position.1;

                // Checks if the neighbour is within the board
                if x >= 0 && x < 100 && y >= 0 && y < 100 {
                    let x_usize = x as usize;
                    let y_usize = y as usize;
                    let mut neighbour = board[x_usize][y_usize].clone();
                    let clone_neighbour = neighbour.clone();

                    // Checks if the signal is supposed to be updated
                    if (current_signal > 0 && neighbour.get_output() < &current_signal) && (!visited.contains(&(*neighbour.get_x(), *neighbour.get_y())) || (neighbour.get_output() > &0)) {
                        let is_diagonal = !(neighbour.get_x() == node.get_x() || neighbour.get_y() == node.get_y());
                        neighbour.set_input(current_signal, mountain_source, network_type.clone());
                        
                        let output_signal ;

                        // Checks if the neighbour is diagonal to the original node
                        if is_diagonal {
                            output_signal = (*neighbour.get_output() as f32 * network_modifier as f32) as i32;
                        }
                        else {
                            output_signal = *neighbour.get_output();
                        }
                    
                        if output_signal > 0 {
                            // Adds the neighbour to the board
                            visited.insert((neighbour.get_x().clone(), neighbour.get_y().clone()));
                            node_queue.add(neighbour.clone());
                            signal_queue.add(output_signal).unwrap();

                            board[x_usize][y_usize] = neighbour;

                        }
                    }
                    
                    // Sends the neighbour to the client
                    let dto = clone_neighbour.convert_to_dto();
                    let temp = vec![dto];
                    socket.write_message(Message::Text(to_string(&temp).unwrap())).unwrap();
                }

            }

            iteration_count += 1;
        }
    }
}


// Removes a building from the board
fn remove_building(x: i32, y: i32, buildings: &mut Vec<Building>) {
    buildings.retain(|building| building.get_x() != &x || building.get_y() != &y);
}

// Clears the board
fn clear_board(board: &Arc<RwLock<Vec<Vec<Node>>>>, network_type: String) {
    let mut guard = board.write().unwrap();
    for row in guard.iter_mut() {
        for column in row.iter_mut() {
            column.set_input(0, false, network_type.clone());
        }
    }
}

