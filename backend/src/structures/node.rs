//Defines the structure of a node in the graph
struct Node {
    is_perimeter: bool,
    is_changed: bool,
    x: i32,
    y: i32,
    weight: i32,
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
    fn get_coor(&self) -> (&i32, &i32) {
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
    fn set_weight(&mut self) -> &mut i32 {
        &mut self.weight
    }

    //Mutable access to the input to the node
    fn set_input(&mut self) -> &mut i32 {
        &mut self.input
    }

    //Mutable access to the output of the node
    fn set_output(&mut self) -> &mut i32 {
        &mut self.output
    }

}