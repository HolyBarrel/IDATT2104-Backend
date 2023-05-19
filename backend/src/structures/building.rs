// Defines the structure of a node in the graph
#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct Building {
    x: i32,
    y: i32,
    building_type: String,
}

// Implementation of the building structure
impl Building {
    // Creates a new building
    pub fn new(x: i32, y: i32, building_type: String) -> Building {
        Building {
            x,
            y,
            building_type,
        }
    }

    // Returns the x-coordinate of the building
    pub fn get_x(&self) -> &i32 {
        &self.x
    }

    // Returns the y-coordinate of the building
    pub fn get_y(&self) -> &i32 {
        &self.y
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_building_creation() {
        let building = Building::new(10, 20, "Tower".to_string());
        assert_eq!(*building.get_x(), 10);
        assert_eq!(*building.get_y(), 20);
    }
}

