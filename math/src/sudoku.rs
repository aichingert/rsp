// Sudoku solver
// (c) aichigert

#[derive(Debug, Clone)]
pub struct Sudoku {
    board: Vec<Vec<i32>>,
    values: Vec<Vec<Vec<i32>>>,
    dimension: usize,
}

impl Sudoku {
    /// new:
    /// Creates a new Sudoku from a Vec
    pub fn new(puzzle: &Vec<Vec<i32>>) -> Self {
        let mut board: Vec<Vec<i32>> = Vec::new();

        for i in 0..puzzle.len() {
            board.push(vec![]);
            for j in 0..puzzle[i].len() {
                board[i].push(puzzle[i][j]);
            }
        }

        Self { board, values: Vec::new(), dimension: (puzzle.len() as f64).sqrt() as usize }
    }

    /// from_str:
    /// Creates a new Sudoku from a string
    pub fn from_str(string: &str) -> Self {
        let mut board: Vec<Vec<i32>> = Vec::new();
        let mut loc: usize = 0;

        for line in string.lines() {
            board.push(vec![]);
            for ch in line.chars() {
                    board[loc].push(((ch as u8)-('0' as u8)) as i32);
            }
            loc += 1;
        }

        Self { board, values: Vec::new(), dimension: ((loc+1) as f64).sqrt() as usize }
    }

    /// find_values:
    /// gets the possible values for a position[row][col]
    fn find_values(&self, row: usize, col: usize) -> Vec<i32> {
        if self.board[row][col] != 0 {
            return vec![];
        }

        let mut invalid: Vec<i32> = Vec::new();

        // Searches the board vertical and diagonal
        for i in 0..self.board.len() {
            if col != i && !invalid.contains(&self.board[row][i]) && self.board[row][i] != 0 {
                invalid.push(self.board[row][i]);
            }
            if row != i && !invalid.contains(&self.board[i][col]) && self.board[i][col] != 0 {
                invalid.push(self.board[i][col]);
            }
        }


        // Searches the grid the number is in if the the row would be 1 and the col 2
        // the area we are going to search is marked

        //	0 2 | 0 0
        //	1 0 | 3 0
        //          - - -
        //	3 4 0 0
        //	2 3 1 0
        let ry = row / self.dimension;
        let cx = col / self.dimension;

        for r in ry*self.dimension..(ry+1)*self.dimension {
            for c in cx*self.dimension..(cx+1)*self.dimension{
                if self.board[r][c] != 0 && !invalid.contains(&self.board[r][c]) {
                    invalid.push(self.board[r][c]);	
                }
            }
        }

        // Filters every value we found so we get the possible ones
        (1..=(self.dimension*self.dimension)as i32).filter(|n| !invalid.contains(n)).collect::<Vec<i32>>()
    }

    /// set_values:
    /// sets the possible values for every position
    fn set_values(&mut self) {
        self.values = Vec::new();

        for i in 0..self.board.len() {
            self.values.push(vec![]);
            for j in 0..self.board[i].len() {
                let pos = self.find_values(i,j);
                self.values[i].push(pos);
            }
        }
    }

    /// solve:
    /// solves the sudoku
    pub fn solve(&mut self) {
        self.set_obv();
    }

    /// set_obv:
    /// sets the values that have only one possibility
    fn set_obv(&mut self) {
        let mut done: bool = false;

        while !done {
            self.set_values();
            done = true;

            for i in 0..self.board.len() {
                for j in 0..self.board[i].len() {
                    if self.values[i][j].len() == 1 {
                        self.board[i][j] = self.values[i][j][0];
                        done = false;
                    }
                }
            }
        }
    }

    /// solved:
    /// checks if we are done
    fn solved(&self) -> bool {
        for i in 0..self.board.len() {
            for j in 0..self.board[i].len() {
                if self.board[i][j] == 0 { return false; }
            }
        }

        true
    }

    pub fn show(&self) {
        for i in 0..self.board.len() {
            print!("\x1B[90m[ ");
            for j in 0..self.board[i].len() {
                match self.board[i][j] {
                    0 => print!("\x1B[30m{} ", self.board[i][j]),
                    1 => print!("\x1B[95m{} ", self.board[i][j]),
                    2 => print!("\x1B[33m{} ", self.board[i][j]),
                    3 => print!("\x1B[93m{} ", self.board[i][j]),
                    4 => print!("\x1B[34m{} ", self.board[i][j]),
                    5 => print!("\x1B[94m{} ", self.board[i][j]),
                    6 => print!("\x1B[36m{} ", self.board[i][j]),
                    7 => print!("\x1B[96m{} ", self.board[i][j]),
                    8 => print!("\x1B[36m{} ", self.board[i][j]),
                    9 => print!("\x1B[96m{} ", self.board[i][j]),
                    _ => print!("{} ", self.board[i][j]),
                };
            }
            println!("\x1B[90m]");
        }
    }
}
