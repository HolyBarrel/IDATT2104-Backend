//Defines the structure of a node in the graph
struct Node {
    isPerimeter: bool,
    isChanged: bool,
    x: i32,
    y: i32,
    weight: i32,
    output: i32,
}

//Implementation of the node structure
impl Node {

    //Returns true if the node is on the perimeter
    fn isPerimeter(&self) -> &bool {
        &self.isPerimeter
    }

    //Returns true if the node has been changed
    fn isChanged(&self) -> &bool {
        &self.isChanged
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

    //Returns the output of the node
    fn get_output(&self) -> &i32 {
        &self.output
    }

    //Mutable access to the perimeter status of the node
    fn isPerimeter_mut(&mut self) -> &mut bool {
        &mut self.isPerimeter
    }

    //Mutable access to the changed status of the node
    fn isChanged_mut(&mut self) -> &mut bool {
        &mut self.isChanged
    }

    //Mutable access to the x coordinate of the node
    fn get_x_mut(&mut self) -> &mut i32 {
        &mut self.x
    }

    //Mutable access to the y coordinate of the node
    fn get_y_mut(&mut self) -> &mut i32 {
        &mut self.y
    }

    //Mutable access to the coordinate of the node
    fn get_coor_mut(&mut self) -> (&mut i32, &mut i32) {
        (&mut self.x, &mut self.y)
    }

    //Mutable access to the weight of the node
    fn get_weight_mut(&mut self) -> &mut i32 {
        &mut self.weight
    }

    //Mutable access to the output of the node
    fn get_output_mut(&mut self) -> &mut i32 {
        &mut self.output
    }

}