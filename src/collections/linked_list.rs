use std::fmt::Display;

#[derive(Debug)]
pub struct LinkedList<T: Display> {
    head: Option<Node<T>>,
    len: usize
}

impl<T: Display> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            len: 0
        }
    }

    pub fn from_nodes() -> Self {
        Self {
            head: None,
            len: 0
        }
    }
}


#[derive(Debug)]
pub struct Node<T: Display> {
    value: T,
    next: Option<Box::<Node<T>>>
}

impl<T: Display> Node<T> {
    pub fn new(value: T) -> Self {
        Node {
            value,
            next: None,
        }

    }

}
