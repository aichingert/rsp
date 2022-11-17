use std::fmt::{self, Display, Debug};

#[derive(Debug)]
pub struct LinkedList {
    head: Link
}

#[derive(Debug)]
enum Link {
    Next(Box<Node>),
    Null 
}

#[derive(Debug)]
struct Node {
    value: i32,
    next: Link
}

impl LinkedList {
    pub fn new() -> Self {
        Self {
            head: Link::Null
        }
    }
}


