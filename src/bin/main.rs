use rust_playground::collections::linked_list::*;

fn main() {
    let mut node: LinkedList<i32> = LinkedList::new(10);
    node.push(10);
    node.push(20);
    node.push(10);

    println!("{}", node);
}
