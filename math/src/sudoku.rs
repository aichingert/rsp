// Sudoku solver
// (c) aichigert

#[derive(Debug, Clone)]
pub struct Sudoku {
    board: Vec<Vec<i32>>,
    values: Vec<Vec<Vec<i32>>>,
    dimension: usize,
}

impl Sudoku {
    pub fn new(puzzle: &Vec<Vec<i32>>) -> Self {
        Self { 
            board: puzzle.clone(), 
            values: Vec::new(), 
            dimension: (puzzle.len() as f64).sqrt() as usize 
        }
    }

    pub fn from_str(s: &str) -> Self {
        let board = s.lines()
            .map(|l| l.chars().map(|c| (c as u8 - b'0') as i32).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let len = board.len() as f64;

        Self { board, values: Vec::new(), dimension: len.sqrt() as usize }
    }

    fn find_values(&self, row: usize, col: usize) -> Vec<i32> {
        if self.board[row][col] != 0 {
            return vec![];
        }

        let mut invalid: Vec<i32> = Vec::new();

        for i in 0..self.board.len() {
            if col != i && !invalid.contains(&self.board[row][i]) && self.board[row][i] != 0 {
                invalid.push(self.board[row][i]);
            }
            if row != i && !invalid.contains(&self.board[i][col]) && self.board[i][col] != 0 {
                invalid.push(self.board[i][col]);
            }
        }

        let ry = row / self.dimension;
        let cx = col / self.dimension;

        for r in ry*self.dimension..(ry+1)*self.dimension {
            for c in cx*self.dimension..(cx+1)*self.dimension{
                if self.board[r][c] != 0 && !invalid.contains(&self.board[r][c]) {
                    invalid.push(self.board[r][c]);	
                }
            }
        }

        (1..=(self.dimension*self.dimension) as i32).filter(|n| !invalid.contains(n)).collect::<Vec<i32>>()
    }

    fn set_values(&mut self) {
        self.values.clear();

        for i in 0..self.board.len() {
            self.values.push(vec![]);
            for j in 0..self.board[i].len() {
                let pos = self.find_values(i,j);
                self.values[i].push(pos);
            }
        }
    }

    fn get_pos(sudoku: &Sudoku) -> Option<(usize,usize, Vec<i32>)> {
        for i in 0..sudoku.values.len() {
            for j in 0..sudoku.values[i].len() {
                if sudoku.values[i][j].len() == 2 {
                    return Some((i, j, sudoku.values[i][j].clone()));
                }
            }
        }

        None
    }

    pub fn solve(&mut self) {
        self.set_obv();
        
        while !self.is_solved() {
            self.set_values();

            match Self::get_pos(self) {
                Some(pos) => {
                    let mut me = self.clone();

                    match Self::check_number(&mut me, (pos.0,pos.1), pos.2[0]) {
                        true => self.board = me.board,
                        false => self.board[pos.0][pos.1] = pos.2[1],
                    }
                }
                None => return
            };

            self.set_obv();
        }     
    }

    fn check_number(sudoku: &mut Sudoku, cur: (usize, usize), num: i32)-> bool {
        sudoku.board[cur.0][cur.1] = num;
        sudoku.set_obv();

        if sudoku.is_solved() {
            return true;
        }

        match Self::get_pos(sudoku) {
            Some(pos) => {
                let mut inner = sudoku.clone();

                match Self::check_number(&mut inner, (pos.0, pos.1), pos.2[0]) {
                    true => sudoku.board = inner.board,
                    false => {
                        let mut iknow = sudoku.clone();

                        match Self::check_number(&mut iknow, (pos.0, pos.1), pos.2[1]) {
                            true => sudoku.board = iknow.board,
                            false => return false,
                        }
                    }
                }

                true
            }
            None => false,
        }
    }

    fn set_obv(&mut self) {
        let mut done: bool = false;

        while !done {
            self.set_values();
            done = true;

            for i in 0..self.board.len() {
                for j in 0..self.board[i].len() {
                    if self.values[i][j].len() == 1 {
                        self.board[i][j] = self.values[i][j][0];
                        self.set_values();
                        done = false;
                    }
                }
            }
        }
    }

    fn is_solved(&self) -> bool {
        !self.board.iter().any(|b| b.iter().any(|&x| x == 0))
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


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sudoku_find_values_with_one_pos() {
        let sudoku = Sudoku::from_str("000000000\n000000000\n000000000\n000056000\n000123000\n000478000\n000000000\n000000000\n000000000");

        assert_eq!(vec![9], sudoku.find_values(3, 3));
    }

    #[test]
    fn sudoku_find_values_with_multiple_pos() {
        let sudoku = Sudoku::from_str("000900000\n000000000\n000000000\n000056000\n000103000\n000480000\n000000000\n000000000\n000000000");

        assert_eq!(vec![2,7], sudoku.find_values(3, 3));
        assert_eq!(vec![2,7,9], sudoku.find_values(5, 5));
        assert_eq!(Vec::<i32>::new(), sudoku.find_values(3,5));
    }

    #[test]
    fn sudoku_find_values_with_row_col_pos() {
        let sudoku = Sudoku::from_str("000010000\n000020000\n000030000\n000000000\n540000009\n000800000\n000000000\n000000000\n000070000");

        assert_eq!(vec![6], sudoku.find_values(4, 4));
    }
    
    #[test]
    fn sudoku_find_values_with_failing_example() {
        let sudoku = Sudoku::from_str("531600487\n640800052\n000500000\n006100230\n010306040\n700200091\n900400070\n000760000\n000908005");

        assert_eq!(vec![8], sudoku.find_values(3, 8));
        assert_eq!(vec![8], sudoku.find_values(4, 8));
    }
}

