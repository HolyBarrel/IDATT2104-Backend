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
        let mut answer_node = Node::new(*&self.x, *&self.y);
        match self.landscape.clone() {
            Some(value)=>{
                answer_node.set_landscape(self.landscape.clone().unwrap());
            }
            None=>{
                answer_node.set_landscape("field".to_string());
            }
        }
        match self.building.clone() {
            Some(value)=>{
                answer_node.set_landscape(self.building.clone().unwrap());
            }
            None=>{
                answer_node.set_landscape("none".to_string());
            }
        }
        return answer_node;
    }
}

#[derive(Serialize,Deserialize,Debug)]
pub struct  answerDTO{
    pub x:i32,
    pub y:i32,
    pub power: f32
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_dto_getters() {
        let node_dto = NodeDTO {
            x: 10,
            y: 20,
            landscape: Some("Forest".to_string()),
            building: Some("House".to_string()),
        };

        assert_eq!(node_dto.get_x(), &10);
        assert_eq!(node_dto.get_y(), &20);
        assert_eq!(node_dto.get_landscape(), Some("Forest".to_string()));
        assert_eq!(node_dto.get_building(), Some("House".to_string()));
    }

    #[test]
    fn test_node_dto_get_node() {
        let node_dto = NodeDTO {
            x: 10,
            y: 20,
            landscape: Some("Forest".to_string()),
            building: Some("House".to_string()),
        };

        let node = node_dto.get_node();

        assert_eq!(node.get_x(), &10);
        assert_eq!(node.get_y(), &20);
    }

    #[test]
    fn test_answer_dto_new() {
        let answer_dto = answerDTO::new(10, 20);

        assert_eq!(answer_dto.x, 10);
        assert_eq!(answer_dto.y, 20);
        assert_eq!(answer_dto.power, 30.0);
    }
}
