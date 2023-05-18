use std::thread;
use std::sync::{Arc, Mutex};
use serde::de::Error;
use serde_json::{Result, from_str, to_string};
use tungstenite::{WebSocket, Message, accept};
use std::net::{TcpListener,TcpStream};
mod structures;
use structures::dto::NodeDTO;
use structures::node_queue::NodeQueue;
use structures::node::Node;
use structures::building::Building;
use std::collections::HashSet;
use structures::dto::answerDTO;
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use queues::*;

fn main() {
    let server = TcpListener::bind("127.0.0.1:8765").unwrap();
    let connections = Arc::new(Mutex::new(Vec::new()));

    for stream in server.incoming() {
        let connections = connections.clone();
        let stream = stream.unwrap();

        
        let socket = Arc::new(Mutex::new(accept(stream.try_clone().unwrap()).unwrap()));
        let socket_clone = socket.clone();

        thread::spawn(move || {
            let mut socket = accept(stream).unwrap();
            connections.lock().unwrap().push(socket.get_ref().peer_addr().unwrap());
            handle_connection(socket_clone, connections.clone());
            // Remove the connection from the list when the thread is finished
            let mut connections = connections.lock().unwrap();
            connections.retain(|c| *c != socket.get_ref().peer_addr().unwrap());
        });
    }
}

fn handle_connection(socket: Arc<Mutex<WebSocket<TcpStream>>>, connections: Arc<Mutex<Vec<std::net::SocketAddr>>>) {
    let network_type = Arc::new(RwLock::new("5G".to_string()));
    let vec_buildings: Vec<Building> = vec![];
    let antennas = Arc::new(RwLock::new(vec_buildings.clone()));
    let extenders = Arc::new(RwLock::new(vec_buildings));
    let board_lock = Arc::new(RwLock::new(vec![vec![Node::new(-1, -1); 100]; 100]));
    let mut socket_copy = socket.clone();

    while let Ok(msg) = socket_copy.lock().unwrap().read_message() {
        let network_type_clone = network_type.clone();

        if msg.is_binary() || msg.is_text() {
            let mut msg_clone = msg.clone();
            let text_msg = msg_clone.to_text().unwrap();

            if text_msg.starts_with("?") {
                let sub_text = &text_msg[1..];
                let mut guard = network_type_clone.write().unwrap();
                *guard = sub_text.to_string();
            }

            let nodes = match parse_json(msg) {
                Ok(nodes) => nodes,
                Err(_) => continue,
            };

            let mut node_clone = vec![];

            for node in nodes {
                node_clone.push(node.clone());

                match node.get_building() {
                    Some(value) => {
                        if value.eq_ignore_ascii_case("antenna") {
                            let mut guard = antennas.write().unwrap();
                            guard.push(Building::new(*node.get_x(), *node.get_y(), value));
                        } else {
                            let mut guard = extenders.write().unwrap();
                            guard.push(Building::new(*node.get_x(), *node.get_y(), value));
                        }
                    },
                    None => {
                        let mut guard = antennas.write().unwrap();
                        let mut copy = guard.clone();
                        remove_building(*node.get_x(), *node.get_y(), &mut copy);
                        guard.clear();
                        guard.extend(copy);

                        let mut guard = extenders.write().unwrap();
                        let mut copy = guard.clone();
                        remove_building(*node.get_x(), *node.get_y(), &mut copy);
                        guard.clear();
                        guard.extend(copy);
                    },
                }
            }

            {
                clean_board(&board_lock);
                let guard = board_lock.read().unwrap();
                let clean_board = convert_board_to_dto(&guard);
                let mut socket = socket_copy.lock().unwrap();
                socket.write_message(Message::Text(to_string(&clean_board).unwrap())).unwrap();

                socket.write_message(Message::Text(to_string(&clean_board).unwrap())).unwrap();
                let _queue = NodeQueue::new_queue();
            }

            {
                let mut guard = board_lock.write().unwrap();
                populate_board(&mut guard, node_clone);
            }

            let building_guard = antennas.read().unwrap();
            let mut threads = Vec::new();

            for x in building_guard.iter() {
                let copy_guard = board_lock.clone();
                let read_guard = board_lock.read().unwrap();
                let node = read_guard[*x.get_x() as usize][*x.get_y() as usize].clone();
                let connections = connections.clone();
                let mut WebSocket = socket_copy.clone();
                
                threads.push(thread::spawn(move || {
                    

                    spread_signal(node, &copy_guard, &mut WebSocket, connections);
                }));
            }

            for thread in threads {
                let _ = thread.join();
            }

            let building_guard: RwLockReadGuard<Vec<Building>> = extenders.read().unwrap();
            let mut threads = Vec::new();
            for x in building_guard.iter() {
                let copy_guard = board_lock.clone();
                let read_guard = board_lock.read().unwrap();
                let node = read_guard[*x.get_x() as usize][*x.get_y() as usize].clone();
                let connections = connections.clone();
                let mut WebSocket = socket_copy.clone();
                
                threads.push(thread::spawn(move || {
                    spread_signal(node, &copy_guard, &mut WebSocket, connections);
                }));
            }

            for thread in threads {
                let _ = thread.join();
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
        _ => Err(Error::custom("Not json parsable!")),
    }
}

fn convert_board_to_dto(board: &RwLockReadGuard<Vec<Vec<Node>>>) -> Vec<answerDTO> {
    let mut nodes_dto = Vec::new();

    for row in board.iter() {
        for node in row.iter() {
            let answer_dto = node.convert_to_DTO();
            nodes_dto.push(answer_dto);
        }
    }

    nodes_dto
}

fn populate_board(board: &mut RwLockWriteGuard<Vec<Vec<Node>>>, nodes: Vec<NodeDTO>) {
    for node in nodes {
        let x = *node.get_x() as usize;
        let y = *node.get_y() as usize;
        let landscape = node.get_landscape().unwrap_or_else(|| "field".to_string());
        let building = node.get_building().unwrap_or_else(|| "none".to_string());

        let mut node = Node::new_from_usize(x, y);
        node.set_landscape(landscape.to_string());
        node.set_building(building.to_string());
        node.set_weight();
        board[x][y] = node;
    }
}

fn spread_signal(
    mut node: Node,
    board: &Arc<RwLock<Vec<Vec<Node>>>>,
    socket: &mut Arc<Mutex<WebSocket<TcpStream>>>,
    connections: Arc<Mutex<Vec<std::net::SocketAddr>>>,
) {
    let mut node_queue = NodeQueue::new_queue();
    let mut signal_queue = Queue::<i32>::new();
    let mut visited = HashSet::<Node>::new();
    let mut guard = board.write().unwrap();
    let mut WebSocket = socket.lock().unwrap();

    print!("This is the building: {:?}", node.get_building());

    node.set_output(100);

    let mut mountain_source = false;
    if node.get_landscape() == "mountain" {
        mountain_source = true;
    }

    guard[*node.get_x() as usize][*node.get_y() as usize] = node.clone();
    let dto = node.convert_to_DTO();
    let temp = vec![dto];
    WebSocket.write_message(Message::Text(to_string(&temp).unwrap())).unwrap();

    node_queue.add(node.clone());
    signal_queue.add(*node.get_output());
    visited.insert(node.clone());

    let mut iteration_count = 0;

    while node_queue.size() > 0 && iteration_count < 100000 {
        if let Ok(current_node) = node_queue.pop_first() {
            let current_signal = signal_queue.remove().unwrap();
            let neighbour_positions = current_node.adj_positions();

            for position in neighbour_positions {
                let x = position.0;
                let y = position.1;

                if x >= 0 && x < 100 && y >= 0 && y < 100 {
                    let x_usize = x as usize;
                    let y_usize = y as usize;
                    let mut neighbour = guard[x_usize][y_usize].clone();
                    let mut clone_neighbour = neighbour.clone();

                    if (current_signal > 0 && neighbour.get_output() < &current_signal)
                        && (!visited.contains(&neighbour) || neighbour.get_output() > &0)
                    {
                        neighbour.set_input(current_signal, mountain_source);
                        let output_signal = neighbour.get_output();

                        if output_signal > &0 {
                            visited.insert(neighbour.clone());
                            node_queue.add(neighbour.clone());
                            signal_queue.add(*output_signal);

                            guard[x_usize][y_usize] = neighbour;
                        }
                    }

                    let dto = node.convert_to_DTO();
                    let temp = vec![dto];
                    WebSocket.write_message(Message::Text(to_string(&temp).unwrap())).unwrap();

                }
            }

            iteration_count += 1;
        }
    }

    // Send the updated connections list to the client
    let connections_guard = connections.lock().unwrap();
    let connections_dto = connections_guard.iter().map(|c| c.to_string()).collect::<Vec<String>>();
    let connections_msg = Message::Text(to_string(&connections_dto).unwrap());
    WebSocket.write_message(connections_msg).unwrap();
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
