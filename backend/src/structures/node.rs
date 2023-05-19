use super::dto::answerDTO;
use std::{hash::{Hash, Hasher}, cmp::Ordering};

//Defines the structure of a node in the graph
#[derive(Clone,Debug)]
pub struct Node {
    is_perimeter: bool,
    is_changed: bool,
    x: i32,
    y: i32,
    weight: i32,
    landscape: String,
    building: String,
    input: i32,
    output: i32,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.weight.cmp(&other.weight)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
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
            building: String::from("none"),
            input: 0,
            output: 0,

        }
    }

        //Creates a new node from usizes
        pub fn new_from_usize(x: usize, y: usize) -> Node {
            Node {
                is_perimeter: false,
                is_changed: false,
                x: x as i32,
                y: y as i32,
                weight: 0,
                landscape: String::from("field"),
                building: String::from("none"),
                input: 0,
                output: 0,
    
            }
        }
    

    //Creates a new node with a weight
    pub fn new_weighted(x: i32, y: i32, weight: i32) -> Node {
        Node {
            is_perimeter: false,
            is_changed: false,
            x,
            y,
            weight,
            landscape: String::from("field"),
            building: String::from("none"),
            input: 0,
            output: 0,
        }
    }

    pub fn convert_to_DTO(&self) -> answerDTO{
        answerDTO{
            x: self.x,
            y: self.y,
            power: self.output as f32
        }
    }

    //Returns true if the node is on the perimeter
    pub fn is_perimeter(&self) -> &bool {
        &self.is_perimeter
    }

    //Returns true if the node has been changed
    pub fn is_changed(&self) -> &bool {
        &self.is_changed
    }

    //Returns the x coordinate of the node
    pub fn get_x(&self) -> &i32 {
        &self.x
    }

    //Returns the y coordinate of the node
    pub fn get_y(&self) -> &i32 {
        &self.y
    }

    //Returns the coordinate of the node
    pub fn get_coor(&self) -> (&i32, &i32) {
        (&self.x, &self.y)
    }

    //Returns the weight of the node
    pub fn get_weight(&self) -> &i32 {
        &self.weight
    }

    //Returns the landscape of the node
    pub fn get_landscape(&self) -> &String {
        &self.landscape
    }

    //Returns the building of the node
    pub fn get_building(&self) -> &String {
        &self.building
    }

    // Returns the input to the node
    pub fn get_input(&self) -> &i32 {
        &self.input
    }

    //Returns the output from the node
    pub fn get_output(&self) -> &i32 {
        &self.output
    }

    //Mutable access to the perimeter status of the node
    pub fn set_is_perimeter(&mut self) -> &mut bool {
        &mut self.is_perimeter
    }

    //Mutable access to the changed status of the node
    pub fn set_is_changed(&mut self) -> &mut bool {
        &mut self.is_changed
    }

    //Mutable access to the x coordinate of the node
    pub fn set_x(&mut self) -> &mut i32 {
        &mut self.x
    }

    //Mutable access to the y coordinate of the node
    pub fn set_y(&mut self) -> &mut i32 {
        &mut self.y
    }

    //Mutable access to the coordinate of the node
    pub fn set_coor(&mut self) -> (&mut i32, &mut i32) {
        (&mut self.x, &mut self.y)
    }

    //Mutable access to the weight of the node
    pub fn set_weight(&mut self) -> &mut i32 {
        //Sets the weight of the node based on the landscape
        self.weight = match self.landscape.as_str() {
            "mountain" => 80, //TODO: Change to real data
            "forest" => 24,
            "water" => 8,
            "field" => 12,
            "city" => 26,
            _ => 0,
        };
        &mut self.weight
    }

    //Sets the landscape of the node
    pub fn set_landscape(&mut self, landscape: String) -> &mut String {
        self.landscape = landscape;
        &mut self.landscape
    }
    

    //Sets the building of the node
    pub fn set_building(&mut self, building: String) -> &mut String {
        self.building = building;
        &mut self.building
    }

    //Sets the input of the node
    pub fn set_input(&mut self, input: i32, mountain_source: bool, network_type: String) -> &mut i32 {
        self.input = input;
        //Sets the output of the node based on the input and weight
        let mut actual_weight = self.weight;

        if network_type == "4G" {
            if self.landscape == "mountain" {
                actual_weight = (self.weight as f32 * 0.7 as f32) as i32;
            }
            else if self.landscape == "forest" || self.landscape == "city" {
                actual_weight = (self.weight as f32 * 0.5 as f32) as i32;
            }
            else {
                actual_weight = (self.weight as f32 * 0.25 as f32) as i32;
            }
        }
        else if network_type == "3G" {
            if self.landscape == "mountain" {
                actual_weight = (self.weight as f32 * 0.5 as f32) as i32;
            }
            else if self.landscape == "forest" || self.landscape == "city" {
                actual_weight = (self.weight as f32 * 0.3 as f32) as i32;
            }
            else {
                actual_weight = (self.weight as f32 * 0.05 as f32) as i32;
            }
        }

        if actual_weight < 1 {
            actual_weight = 1;
        }

        if self.input < 2 {
            self.output = 0;
        } else {
            if (mountain_source && self.landscape == "mountain") && ((self.input - actual_weight/6) > self.output)  {
                self.output = self.input - (actual_weight / 6);
            }
            else if (self.input - actual_weight) > self.output {
                self.output = self.input - actual_weight;
            } else {
                self.output = 0;
            }
            //self.output = self.input - (self.weight);
            if self.output < 0 {
                self.output = 0;
            }
        }

        &mut self.input
    }

    //Sets the output of the node
    pub fn set_output(&mut self, output: i32) -> &mut i32 {
        if output < 0 {
            self.output = 0;
        } else if output > 100 {
            self.output = 100;
        } else {
            self.output = output;
        }
        
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

//Implements the PartialEq, Eq and Hash traits for the Node struct
impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Node {}

impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let node = Node::new(1, 2);

        assert_eq!(node.is_perimeter(), &false);
        assert_eq!(node.is_changed(), &false);
        assert_eq!(node.get_x(), &1);
        assert_eq!(node.get_y(), &2);
        assert_eq!(node.get_weight(), &0);
        assert_eq!(node.get_landscape(), &String::from("field"));
        assert_eq!(node.get_building(), &String::from("none"));
        assert_eq!(node.get_input(), &0);
        assert_eq!(node.get_output(), &0);
    }

    #[test]
    fn test_new_from_usize() {
        let node = Node::new_from_usize(1, 2);

        assert_eq!(node.is_perimeter(), &false);
        assert_eq!(node.is_changed(), &false);
        assert_eq!(node.get_x(), &1);
        assert_eq!(node.get_y(), &2);
        assert_eq!(node.get_weight(), &0);
        assert_eq!(node.get_landscape(), &String::from("field"));
        assert_eq!(node.get_building(), &String::from("none"));
        assert_eq!(node.get_input(), &0);
        assert_eq!(node.get_output(), &0);
    }

    #[test]
    fn test_new_weighted() {
        let node = Node::new_weighted(1, 2, 5);

        assert_eq!(node.is_perimeter(), &false);
        assert_eq!(node.is_changed(), &false);
        assert_eq!(node.get_x(), &1);
        assert_eq!(node.get_y(), &2);
        assert_eq!(node.get_weight(), &5);
        assert_eq!(node.get_landscape(), &String::from("field"));
        assert_eq!(node.get_building(), &String::from("none"));
        assert_eq!(node.get_input(), &0);
        assert_eq!(node.get_output(), &0);
    }

    #[test]
    fn test_convert_to_DTO() {
        let node = Node::new_weighted(1, 2, 5);
        let dto = node.convert_to_DTO();

        assert_eq!(dto.x, 1);
        assert_eq!(dto.y, 2);
        assert_eq!(dto.power, 0.0);
    }

    #[test]
    fn test_set_is_perimeter() {
        let mut node = Node::new(1, 2);
        *node.set_is_perimeter() = true;

        assert_eq!(node.is_perimeter(), &true);
    }

    #[test]
    fn test_set_is_changed() {
        let mut node = Node::new(1, 2);
        *node.set_is_changed() = true;

        assert_eq!(node.is_changed(), &true);
    }

    #[test]
    fn test_set_x() {
        let mut node = Node::new(1, 2);
        *node.set_x() = 3;

        assert_eq!(node.get_x(), &3);
    }

    #[test]
    fn test_set_y() {
        let mut node = Node::new(1, 2);
        *node.set_y() = 4;

        assert_eq!(node.get_y(), &4);
    }

    #[test]
    fn test_set_coor() {
        let mut node = Node::new(1, 2);
        *node.set_coor().0 = 3;
        *node.set_coor().1 = 4;

        assert_eq!(node.get_coor(), (&3, &4));
    }

    #[test]
    fn test_set_weight() {
        let mut node = Node::new(1, 2);
        *node.set_weight() = 5;

        assert_eq!(node.get_weight(), &5);
    }

    #[test]
    fn test_set_landscape() {
        let mut node = Node::new(1, 2);
        node.set_landscape(String::from("mountain"));

        assert_eq!(node.get_landscape(), &String::from("mountain"));
    }

    #[test]
    fn test_set_building() {
        let mut node = Node::new(1, 2);
        node.set_building(String::from("house"));

        assert_eq!(node.get_building(), &String::from("house"));
    }

    #[test]
    /* TODO: UPDATE
    fn test_set_input() {
        let mut node = Node::new(1, 2);
        *node.set_input(3, false) = 3;
    
        assert_eq!(node.get_input(), &3);
        assert_eq!(node.get_output(), &3);  // Update the expected output value
    }
    */

    #[test]
    fn test_set_output() {
        let mut node = Node::new(1, 2);
        *node.set_output(4) = 4;

        assert_eq!(node.get_output(), &4);
    }

    #[test]
    fn test_adj_positions() {
        let node = Node::new(1, 2);
        let adj_positions = node.adj_positions();

        assert_eq!(adj_positions, vec![(0, 2), (0, 1), (0, 3), (2, 2), (2, 1), (2, 3), (1, 1), (1, 3)]);
    }
}
