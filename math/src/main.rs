// Math - rust
// aichingert

pub mod pi;
pub mod random;
pub mod sudoku;
pub mod matrix;

use pi::approximate_pi;
use sudoku::Sudoku;
use matrix::Matrix;

fn main() {

    let a = Matrix::<i32, 2>::new();

    let v = vec![
        vec![2,-2,-1,3],
        vec![4,3,1,-2],
        vec![-1,2,1,-1],
        vec![3,-2,-4,5],
    ];

    println!("RESULT {:?}", matrix::determinant(v));

    /*
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
    // let mut sudoku: Sudoku = Sudoku::from_str("3000\n0020\n0100\n0002");

    sudoku.show();
    sudoku.solve();
    println!();
    sudoku.show();
    */
}
