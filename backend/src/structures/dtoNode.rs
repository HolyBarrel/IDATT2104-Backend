use serde::{Serialize, Deserialize};
//Defines the structure of a node in the graph

#[derive(Serialize, Deserialize)]
pub struct NodeDTO{
    x: i32,
    y: i32,
    landscape: Option<String>,
    building: Option<String>
}