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
        self.queue.add(node).unwrap();
    }

    // Removes a node from the front of the queue and returns it
    pub fn pop_first(&mut self) -> Result<Node, &str> {
        self.queue.remove()
    }

    //Returns the size of the queue
    pub fn size(&self) -> usize {
        self.queue.size()
    }


}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::structures::node::Node;

    #[test]
    fn test_new_queue() {
        let node_queue = NodeQueue::new_queue();

        assert_eq!(node_queue.size(), 0);
    }

    #[test]
    fn test_add() {
        let mut node_queue = NodeQueue::new_queue();
        let node = Node::new(1, 2);
        node_queue.add(node.clone());

        assert_eq!(node_queue.size(), 1);
    }

    #[test]
    fn test_pop_first() {
        let mut node_queue = NodeQueue::new_queue();
        let node1 = Node::new(1, 2);
        let node2 = Node::new(3, 4);
        node_queue.add(node1.clone());
        node_queue.add(node2.clone());

        let popped_node = node_queue.pop_first().unwrap();

        assert_eq!(popped_node, node1);
        assert_eq!(node_queue.size(), 1);
    }

    #[test]
    fn test_pop_first_empty_queue() {
        let mut node_queue = NodeQueue::new_queue();

        let result = node_queue.pop_first();

        assert!(result.is_err());
        assert_eq!(node_queue.size(), 0);
    }

    #[test]
    fn test_size() {
        let mut node_queue = NodeQueue::new_queue();
        let node1 = Node::new(1, 2);
        let node2 = Node::new(3, 4);
        node_queue.add(node1.clone());
        node_queue.add(node2.clone());

        assert_eq!(node_queue.size(), 2);
    }


}
