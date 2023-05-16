//Defines the structure of a node in the graph
pub struct Building {
    x: i32,
    y: i32,
    structure_type: String,
}

//Implementation of the building structure
impl Building {

    //Creates a new building
    pub fn new(x: i32, y: i32, structure_type: String) -> Building {
        Building {
            x,
            y,
            structure_type,
        }
    }

    //Returns the x coordinate of the building
    pub fn get_x(&self) -> &i32 {
        &self.x
    }

    //Returns the y coordinate of the building
    pub fn get_y(&self) -> &i32 {
        &self.y
    }

    //Returns the type of the building
    pub fn get_type(&self) -> &String {
        &self.structure_type
    }

    //Sets the x coordinate of the building
    pub fn set_x(&mut self, x: i32) {
        self.x = x;
    }

    //Sets the y coordinate of the building
    pub fn set_y(&mut self, y: i32) {
        self.y = y;
    }

    //Sets the type of the building
    pub fn set_type(&mut self, structure_type: String) {
        self.structure_type = structure_type;
    }


}
