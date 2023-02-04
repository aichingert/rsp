// Math - rust
// aichingert

mod random;

use random::Rand;

fn main() {
    let mut r = Rand::new(10).into_iter();
    
    println!("{}", r.next().unwrap());
    println!("{}", r.next().unwrap());
}
