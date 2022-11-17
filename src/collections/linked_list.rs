use std::fmt::Display;
use std::mem;

type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LinkedList<T: Display + Clone> {
    head: Link<T>
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Node<T: Display + Clone> {
    elem: T,
    next: Link<T>
}

impl<T: Display + Clone> LinkedList<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn push(&mut self, elem: T) {
        let next: Box<Node<T>> = Box::new(Node {
            elem,
            next: mem::replace(&mut self.head, None)
        });

        self.head = Some(next);
    }

    pub fn pop(&mut self) -> Option<T> {
        match mem::replace(&mut self.head, None) {
            None => None,
            Some(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}

impl<T: Display + Clone> Drop for LinkedList<T> {
    fn drop(&mut self) {
        let mut current: Link<T> = mem::replace(&mut self.head, None);
        while let Some(mut node) = current {
            current = mem::replace(&mut node.next, None);
        }
    }
}

#[cfg(test)]
mod test {
    use super::{LinkedList, Node};
    use crate::linked_list::Link;

    #[test]
    fn new_list_should_be_none() {
        let list: LinkedList<i32> = LinkedList::new();
        assert_eq!(list.head, None)
    }

    #[test]
    fn push_test() {
        let mut list: LinkedList<char> = LinkedList::new();
        list.push('a');
        list.push('b');

        assert_eq!(list.pop(), Some('b'));
    }
}
