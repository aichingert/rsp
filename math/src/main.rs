// Math - rust
// aichingert

pub mod random;
pub mod pi;

use pi::approximate_pi;

fn main() {
    let precision: u32 = 1000000;

    println!("Approximation of pi: {} [precision: {}]", approximate_pi(precision), precision);
}
