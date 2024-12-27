// Math - rust
// aichingert

pub mod kids;
pub mod games;

use kids::DecisionTree;

pub mod pi;
pub mod random;
pub mod sudoku;
pub mod matrix;

use pi::approximate_pi;
use sudoku::Sudoku;
use matrix::Matrix;

fn main() {
    // tuple: (color, value)
    // colors: 
    // heart: 0
    // shell: 1
    // leaf : 2
    // acorn: 3 
    //
    // values: 
    // 2 | 3 | 4 | 10 | 11
    // u | o | k | 10 | ass
    //0
    let result = games::schnapsn::solve(
        0, 
        [(1,3, 0), (3,10,0), (0,10,0), (3,4,0), (0,3,0)],
        [(1,4,0), (0,4,0), (1,2,0), (0,11,0), (2,4,0)],
    );
    println!("{result:?}");

    //let tree = DecisionTree::from("kids/diabetes-final.csv").unwrap();
    //tree.print_tree("");

    /*
    let a = Matrix::<i32, 2>::new();

    let v = vec![
        vec![2,-2,-1,3],
        vec![4,3,1,-2],
        vec![-1,2,1,-1],
        vec![3,-2,-4,5],
    ];

    println!("RESULT {:?}", matrix::determinant(v));
    */

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
