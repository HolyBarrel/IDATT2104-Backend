use serde::{Serialize, Deserialize};
use super::node::Node;
//Defines the structure of a node in the graph

#[derive(Serialize, Deserialize,Clone)]
pub struct NodeDTO{
    x: i32,
    y: i32,
    landscape: Option<String>,
    building: Option<String>
}

impl NodeDTO{
    //Returns the x coordinate of the node
    pub fn get_x(&self) -> &i32 {
        &self.x
    }
    
    //Returns the y coordinate of the node
    pub fn get_y(&self) -> &i32 {
        &self.y
    }

    pub fn get_landscape(&self) ->Option<String>{
        self.landscape.clone()
    }

    pub fn get_building(&self) ->Option<String>{
        self.building.clone()
    }

    pub fn get_node(&self) -> Node{
        Node::new(*&self.x, *&self.y)
    }
}

#[derive(Serialize,Deserialize)]
pub struct  answerDTO{
    x:i32,
    y:i32,
    power: f32
}

impl answerDTO {
    pub fn new(x:i32 , y:i32 ) ->answerDTO{
        answerDTO{
            x,
            y,
            power: (x + y) as (f32)
        }
    }
}