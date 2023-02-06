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

	fn find_values(&self, row: usize, col: usize) -> Vec<i32> {
		if self.board[row][col] != 0 {
			return vec![];
		}

		let mut invalid: Vec<i32> = Vec::new();

		for i in 0..self.board.len() {
			if row != i && !invalid.contains(&self.board[row][i]) && self.board[row][i] != 0 {
				invalid.push(self.board[row][i]);
			}
			if col != i && !invalid.contains(&self.board[i][col]) && self.board[i][col] != 0 {
				invalid.push(self.board[i][col]);
			}
		}
		
		for r in -1..2 {
			for c in -1..2 {
				if r == 0 && c == 0 { continue; }

				let ry = r+(row as i32);
				let cx = c+(col as i32);
				if ry>0 && cx>0 && (ry as usize)<self.board.len() && (cx as usize)<self.board[row].len() 
				&& self.board[ry as usize][cx as usize] != 0 && !invalid.contains(&self.board[ry as usize][cx as usize]) {
					invalid.push(self.board[ry as usize][cx as usize]);	
				}
			}
		}
	
		(1..=9).filter(|n| !invalid.contains(n)).collect::<Vec<i32>>()
	}

	pub fn set_values(&mut self) {
		self.values = Vec::new();
		let mut loc: usize = 0;
		
		for i in 0..self.board.len() {
			self.values.push(vec![]);
			for j in 0..self.board[i].len() {
				let pos = self.find_values(i,j);
				self.values[i].push(pos);
			}
			loc += 1;
		}
	}

	pub fn solve(&mut self) {
		let mut done: bool = false;

		while !done {
			self.set_values();
			done = true;
			
			for i in 0..self.board.len() {
				for j in 0..self.board[i].len() {
					if self.values[i][j].len() == 1 {
						self.board[i][j] = self.values[i][j][0];
						done = false;
						println!("{i} {j}");
					}
				}
			}
		}

		
	}

	fn solved(&self) -> bool {
		for i in 0..self.board.len() {
			for j in 0..self.board[i].len() {
				if board[i][j] == 0 { return false; }
			}
		}

		true
	}
	
	pub fn show(&self) {
		for i in 0..self.board.len() {
			print!("[ ");
			for j in 0..self.board[i].len() {
				print!("{} ", self.board[i][j]);
			}
			println!("]");
		}
	}
}
