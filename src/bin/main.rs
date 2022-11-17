use rust_playground::collections::linked_list::*;

fn main() {
    let mut list: LinkedList = LinkedList::new();

    list.push(10);
    list.push(20);

    println!("{:?}", list.pop());

    println!("{:?}", list);
}
