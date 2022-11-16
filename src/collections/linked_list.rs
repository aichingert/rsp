use std::fmt::{Display, Debug};

#[derive(Debug)]
pub struct LinkedList<T: Display + Debug> {
    head: Option<Box::<Node<T>>>,
    len: usize
}

impl<T: Display + Debug> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            len: 0
        }
    }

    pub fn from_nodes(nodes: &[Node<T>]) -> Self {
        let mut list: LinkedList<T> = LinkedList::new();

        Self {
            head: None,
            len: 0
        }
    }

    pub fn push(&mut self, value: T) {
        if self.head.is_none() {
            self.head = Some(Box::new(Node::new(value)));
            return;
        }

        let mut current = &mut self.head;
        while !current.as_ref().unwrap().next.is_none() {
            current = Some(current.as_ref().unwrap().next).as_mut().unwrap();
        }
            
    }
}


#[derive(Debug)]
pub struct Node<T: Display + Debug> {
    value: T,
    next: Option<Box::<Node<T>>>
}

impl<T: Display + Debug> Node<T> {
    pub fn new(value: T) -> Self {
        Node {
            value,
            next: None,
        }

    }

}
