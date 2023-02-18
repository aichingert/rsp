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

    // Expert --
    //         |
    //         v
    let mut sudoku: Sudoku = Sudoku::from_str("007000003\n159000000\n008000207\n000200046\n040007000\n500800000\n080050900\n000000100\n000091070");

    // 2x2 ------
    //          |
    //          v
    //let mut sudoku: Sudoku = Sudoku::from_str("3000\n0020\n0100\n0002");
    //
    sudoku.show();
    sudoku.solve();
    println!();
    sudoku.show();
}
