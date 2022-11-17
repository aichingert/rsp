use std::fmt::{self, Display, Debug};
use std::mem;

type Link = Option<Box<Node>>;

#[derive(Debug)]
pub struct LinkedList {
    head: Link
}

#[derive(Debug)]
struct Node {
    value: i32,
    next: Link
}

impl LinkedList {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn push(&mut self, elem: i32) {
        let next: Box<Node> = Box::new(Node {
            value: elem,
            next: mem::replace(&mut self.head, None)
        });

        self.head = Some(next);
    }

    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, None) {
            None => None,
            Some(node) => {
                self.head = node.next;
                Some(node.value)
            }
        }
    }
}

