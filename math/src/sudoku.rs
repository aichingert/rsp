// Sudoku solver
// (c) aichigert

#[derive(Debug, Clone)]
pub struct Sudoku {
	board: Vec<Vec<i32>>,
	values: Vec<Vec<Vec<i32>>>,
}

impl Sudoku {
	pub fn new(puzzle: &Vec<Vec<i32>>) -> Self {
		let mut board: Vec<Vec<i32>> = Vec::new();

		for i in 0..puzzle.len() {
			board.push(vec![]);
			for j in 0..puzzle[i].len() {
				board[i].push(puzzle[i][j]);
			}
		}
		
		Self { board, values: Vec::new() }
	}
	
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

		Self { board, values: Vec::new() }
	}
}
