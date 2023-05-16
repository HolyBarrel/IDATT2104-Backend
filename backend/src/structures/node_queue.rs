use crate::structures::node::Node;
use queues::*;
use std::collections::HashSet;

//Defines a node queue struct
struct NodeQueue {
    queue: Queue::<Node>::new(),
}

//Implementation of the node queue struct
impl NodeQueue {
    pub fn new() -> NodeQueue {
        NodeQueue {
        queue: Queue::<Node>::new(),
        }
    }

    //Adds a node to the queue
    pub fn add(&mut self, node: Node) {
        self.queue.add(node);
    }

    //Removes a node from the queue
    pub fn remove(&mut self) -> Option<Node> {
        self.queue.remove()
    }

    //Removes the first node from the queue
    pub fn remove_first(&mut self) -> Option<Node> {
        self.queue.remove_first()
    }

    //Pops the last node from the queue
    pub fn pop(&mut self) -> Option<Node> {
        self.queue.pop()
    }

    //Peek the first node from the queue
    pub fn peek(&self) -> Option<&Node> {
        self.queue.peek()
    }

    //Returns true if the queue is empty
    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    //Returns the length of the queue
    pub fn len(&self) -> usize {
        self.queue.len()
    }
}