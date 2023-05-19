use serde::{Serialize, Deserialize};
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

    
    
}

#[derive(Serialize,Deserialize,Debug)]
pub struct  AnswerDTO{
    pub x:i32,
    pub y:i32,
    pub power: f32
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_dto_getters() {
        let node = NodeDTO {
            x: 10,
            y: 20,
            landscape: Some("Forest".to_string()),
            building: None,
        };

        assert_eq!(*node.get_x(), 10);
        assert_eq!(*node.get_y(), 20);
        assert_eq!(node.get_landscape(), Some("Forest".to_string()));
        assert_eq!(node.get_building(), None);
    }
}
