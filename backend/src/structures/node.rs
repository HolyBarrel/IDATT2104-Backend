use serde::{Serialize, Deserialize};
//Defines the structure of a node in the graph

#[derive(Serialize, Deserialize)]
pub struct Node {
    is_perimeter: bool,
    is_changed: bool,
    x: i32,
    y: i32,
    weight: i32,
    landscape: String,
    input: i32,
    output: i32,
}

//Implementation of the node structure
impl Node {

    //Creates a new node
    pub fn new(x: i32, y: i32) -> Node {
        Node {
            is_perimeter: false,
            is_changed: false,
            x,
            y,
            weight: 0,
            landscape: String::from("field"),
            input: 0,
            output: 0,
        }
    }

    //Creates a new node with a weight
    fn new_weighted(x: i32, y: i32, weight: i32) -> Node {
        Node {
            is_perimeter: false,
            is_changed: false,
            x,
            y,
            weight,
            landscape: String::from("field"),
            input: 0,
            output: 0,
        }
    }

    //Returns true if the node is on the perimeter
    fn is_perimeter(&self) -> &bool {
        &self.is_perimeter
    }

    //Returns true if the node has been changed
    fn is_changed(&self) -> &bool {
        &self.is_changed
    }

    //Returns the x coordinate of the node
    fn get_x(&self) -> &i32 {
        &self.x
    }

    //Returns the y coordinate of the node
    fn get_y(&self) -> &i32 {
        &self.y
    }

    //Returns the coordinate of the node
    pub fn get_coor(&self) -> (&i32, &i32) {
        (&self.x, &self.y)
    }

    //Returns the weight of the node
    fn get_weight(&self) -> &i32 {
        &self.weight
    }

    // Returns the input to the node
    fn get_input(&self) -> &i32 {
        &self.input
    }

    //Returns the output from the node
    fn get_output(&self) -> &i32 {
        &self.output
    }

    //Mutable access to the perimeter status of the node
    fn set_is_perimeter(&mut self) -> &mut bool {
        &mut self.is_perimeter
    }

    //Mutable access to the changed status of the node
    fn set_is_changed(&mut self) -> &mut bool {
        &mut self.is_changed
    }

    //Mutable access to the x coordinate of the node
    fn set_x(&mut self) -> &mut i32 {
        &mut self.x
    }

    //Mutable access to the y coordinate of the node
    fn set_y(&mut self) -> &mut i32 {
        &mut self.y
    }

    //Mutable access to the coordinate of the node
    fn set_coor(&mut self) -> (&mut i32, &mut i32) {
        (&mut self.x, &mut self.y)
    }

    //Mutable access to the weight of the node
    pub fn set_weight(&mut self) -> &mut i32 {
        //Sets the weight of the node based on the landscape
        self.weight = match self.landscape.as_str() {
            "mountain" => 9,
            "forest" => 6,
            "water" => 1,
            "field" => 2,
            "city" => 7,
            _ => 0,
        };
        &mut self.weight
    }

    //Mutable access to the landscape of the node
    fn set_landscape(&mut self, ) -> &mut String {
        &mut self.landscape
    }

    //Mutable access to the input to the node
    fn set_input(&mut self) -> &mut i32 {
        &mut self.input
    }

    //Mutable access to the output of the node
    fn set_output(&mut self) -> &mut i32 {
        &mut self.output
    }

    //Finds the neighbouring nodes positions of this node
    pub fn adj_positions(&self) -> Vec<(i32, i32)> {
        let mut positions: Vec<(i32, i32)> = Vec::new();
        let x = self.x;
        let y = self.y;
        
        //Checks if the node is on the edge of the map
        let left_edge = x == 0;
        let right_edge = x == 99;
        let top_edge = y == 0;
        let bottom_edge = y == 99;

        //If the node is on the edge of the map, only add the neighbouring nodes that are not inside the map
        if !left_edge {
            positions.push((x - 1, y));
            if !top_edge {
                positions.push((x - 1, y - 1));
            }
            if !bottom_edge {
                positions.push((x - 1, y + 1));
            }
        }

        if !right_edge {
            positions.push((x + 1, y));
            if !top_edge {
                positions.push((x + 1, y - 1));
            }
            if !bottom_edge {
                positions.push((x + 1, y + 1));
            }
        }

        if !top_edge {
            positions.push((x, y - 1));
        }

        if !bottom_edge {
            positions.push((x, y + 1));
        }

        positions
    }
        
}

