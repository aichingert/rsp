// Math - rust
// aichingert

pub mod random;
pub mod pi;
pub mod sudoku;

use pi::approximate_pi;
use sudoku::Sudoku;

fn main() {
	// Pi
	let precision: u32 = 1000000;

	println!("Approximation of pi: {} [precision: {}]", approximate_pi(precision), precision);
	
	// Sudoku
	let mut sudoku: Sudoku = Sudoku::from_str("530070000\n600195000");
	println!("{:?}", sudoku);
}
