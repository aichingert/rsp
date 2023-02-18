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

    println!("Approximation of pi: {} [precision: {}] \n", approximate_pi(precision), precision);

    // Sudoku
    // Easy --
    //       |
    //       v
    //let mut sudoku: Sudoku = Sudoku::from_str("530070000\n600195000\n098000060\n800060003\n400803001\n700020006\n060000280\n000419005\n000080079");

    // Expert -
    //        |
    //        v
    let mut sudoku: Sudoku = Sudoku::from_str("031000400\n040000052\n000500000\n006100200\n010306000\n700200091\n900000070\n000060000\n000908005");

    sudoku.show();
    sudoku.solve();
    println!();
    sudoku.show();
}
