use priority_queue::PriorityQueue;
use crate::structures::node::Node;

//Defines a node queue struct
struct NodeQueue {
    queue: PriorityQueue<Node>,
}

//Implementation of the node queue struct
impl NodeQueue {
    pub fn new() -> NodeQueue {
        NodeQueue {
            queue: PriorityQueue::new(),
        }
    }

    pub fn push(&mut self, node: Node) {
        self.queue.push(node, node.weight);
    }

    pub fn pop(&mut self) -> Option<Node> {
        self.queue.pop()
    }

    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }
}