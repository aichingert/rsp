use std::fmt::{self, Display, Debug};
use std::mem;

type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
pub struct LinkedList<T: Display + Clone> {
    head: Link<T>
}

#[derive(Debug)]
struct Node<T: Display + Clone> {
    value: T,
    next: Link<T>
}

impl<T: Display + Clone> LinkedList<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn push(&mut self, elem: T) {
        let next: Box<Node<T>> = Box::new(Node {
            value: elem,
            next: mem::replace(&mut self.head, None)
        });

        self.head = Some(next);
    }

    pub fn pop(&mut self) -> Option<T> {
        match mem::replace(&mut self.head, None) {
            None => None,
            Some(node) => {
                self.head = node.next;
                Some(node.value)
            }
        }
    }
}

