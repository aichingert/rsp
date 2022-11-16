use std::fmt::Display;

pub struct LinkedList {

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
