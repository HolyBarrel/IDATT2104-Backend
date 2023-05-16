use crate::structures::node::Node;
use queues::*;

//Defines a node queue struct
#[derive(Debug)]
pub struct NodeQueue {
    queue: Queue<Node>,
}

//Implementation of the node queue struct
impl NodeQueue {
    pub fn new_queue() -> NodeQueue {
        NodeQueue {
        queue: Queue::<Node>::new(),
        }
    }

    //Adds a node to the queue
    pub fn add(&mut self, node: Node) {
        self.queue.add(node);
    }

    // Removes a node from the front of the queue and returns it
    pub fn pop_first(&mut self) -> Result<Node, &str> {
        self.queue.remove()
    }

    //Returns the size of the queue
    pub fn size(&self) -> usize {
        self.queue.size()
    }

    //Returns the queue
    pub fn get_queue(&self) -> &Queue::<Node> {
        &self.queue
    }
}