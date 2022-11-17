use std::fmt::{self, Display, Debug};

#[derive(Debug, Clone)]
pub struct LinkedList<T: Display + Debug> {
    value: T,
    len: usize,
    next: Node<T>
}

impl<T: Display + Debug> LinkedList<T> {
    pub fn new(value: T) -> Self {
        Self {
            value,
            len: 1,
            next: Node::nil
        }
    }

    pub fn push(&mut self, value: T) {
        match self.next {
            Node::node(ref mut next_node) => {
                next_node.push(value);
            },
            Node::nil => {
                let node = LinkedList::new(value);
                self.next = Node::node(Box::new(node));
            }
        }
    }
}

impl<T: Display + Debug> Display for LinkedList<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.next {
            Node::node(ref next_node) => {
                println!("Node:{}", self.value);
                std::fmt::Display::fmt(&next_node, f)
            },
            Node::nil => {
                Ok(())
            }
        }
    }
}



#[derive(Debug, Clone)]
pub enum Node<T: Display + Debug> {
    node(Box::<LinkedList<T>>),
    nil
}

