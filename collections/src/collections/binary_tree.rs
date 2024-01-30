use std::cmp::Ordering;
use std::cell::RefCell;
use std::rc::Rc;

type TreeNode<T> = Option<Rc<RefCell<Tree<T>>>>;

#[derive(Debug)]
pub struct Tree<T> {
    val: Option<T>,
    lhs: TreeNode<T>,
    rhs: TreeNode<T>,
}

impl<T: Ord> Tree<T> {
    pub fn new() -> Self {
        Self {
            val: None,
            lhs: None,
            rhs: None,
        }
    }

    pub fn from(val: T) -> Self {
        Self {
            val: Some(val),
            lhs: None,
            rhs: None,
        }
    }

    pub fn insert(&mut self, val: T) -> bool {
        if let Some(cur) = &self.val {
            return match cur.cmp(&val) {
                Ordering::Less => {
                    if self.lhs.is_none() {
                        self.lhs = Some(Rc::new(RefCell::new(Tree::from(val))));
                        true
                    } else {
                        self.lhs.as_mut().unwrap().borrow_mut().insert(val)
                    }
                }
                Ordering::Equal=> false,
                Ordering::Greater => {
                    if self.rhs.is_none() {
                        self.rhs= Some(Rc::new(RefCell::new(Tree::from(val))));
                        true
                    } else {
                        self.rhs.as_mut().unwrap().borrow_mut().insert(val)
                    }
                }
            };
        } else {
            self.val = Some(val);
            true
        }
    }

    pub fn remove(&mut self, val: T) -> bool {
        if let Some(cur) = &self.val {
            return match cur.cmp(&val) {
                Ordering::Less => {
                    self.lhs.as_mut().unwrap().borrow_mut().remove(val)
                }
                Ordering::Equal => {
                    true
                }
                Ordering::Greater => {
                    self.rhs.as_mut().unwrap().borrow_mut().remove(val)
                }
            };
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::collections::binary_tree::Tree;

    #[test]
    fn create_new_tree() {
        let tree = Tree::<i32>::new();

        assert_eq!(tree.val, None);
        assert!(tree.lhs.is_none());
        assert!(tree.rhs.is_none());
    }

    #[test]
    fn create_tree_from_value() {
        let tree = Tree::from(0i32);

        assert_eq!(tree.val, Some(0));
        assert!(tree.lhs.is_none());
        assert!(tree.rhs.is_none());
    }

    #[test]
    fn insert_duplicates_into_tree() {
        let mut tree = Tree::<i32>::new();

        assert!(tree.insert(10));
        assert!(tree.insert(20));
        assert!(!tree.insert(20));
    }
}
