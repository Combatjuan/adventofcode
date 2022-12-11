mod advent;

use advent::{advent_exit, Calculator, LineParser, Solution};
use grid::Grid;

type Record = Vec<i8>;
type Answer = i64;

fn static_parse_line(_line_number: usize, s: &String) -> Result<Record, String> {
	let mut row : Vec<i8> = vec![];
	for c in s.chars() {
		let n = c.to_string().parse::<i8>().unwrap();
		row.push(n);
	}
	Ok(row)
}

struct TreeGrid {
	trees: Grid<i8>
}

impl TreeGrid {
	fn new(rows: &Vec<&Vec<i8>>) -> Result<TreeGrid, String> {
		if rows.len() == 0 {
			return Err(format!("No rows available"))
		}
		let row_count = rows[0].len();
		
		let mut trees : Grid<i8> = Grid::new(row_count, 0);
		for (i, row) in rows.iter().enumerate() {
			if row.len() != row_count {
				return Err(format!("Row count on row {} does not match first row {}", i, row_count));
			} else {
				trees.push_row((*row).clone());
			}
		}
		Ok(
			TreeGrid {
				trees
			}
		)
	}

	fn count_visible(&self) -> i64 {
		let row_count = self.trees.rows();
		let column_count = self.trees.cols();
		let mut visible_map : Grid<bool> = Grid::new(row_count, column_count);

		for r in 0..row_count {
			let mut tallest_from_west = -1;
			for c in 0..column_count {
				let tree = self.trees[r][c];
				if tree > tallest_from_west {
					visible_map[r][c] = true;
					tallest_from_west = tree;
				}
			}
		}

		for r in 0..row_count {
			let mut tallest_from_east = -1;
			for c in (0..column_count).rev() {
				let tree = self.trees[r][c];
				if tree > tallest_from_east {
					visible_map[r][c] = true;
					tallest_from_east = tree;
				}
			}
		}

		for c in 0..column_count {
			let mut tallest_from_north = -1;
			for r in 0..row_count {
				let tree = self.trees[r][c];
				if tree > tallest_from_north {
					visible_map[r][c] = true;
					tallest_from_north = tree;
				}
			}
		}

		for c in 0..column_count {
			let mut tallest_from_south = -1;
			for r in (0..row_count).rev() {
				let tree = self.trees[r][c];
				if tree > tallest_from_south {
					visible_map[r][c] = true;
					tallest_from_south = tree;
				}
			}
		}

		visible_map.iter().filter(|x| **x).count() as i64
	}

	fn most_scenic(&self) -> i64 {
		let mut most_scenic = 0;
		for r in 0..self.trees.rows() {
			for c in 0..self.trees.cols() {
				let scenary_score = self.scenary_score_for_tree(r, c);
				if scenary_score > most_scenic {
					most_scenic = scenary_score;
				}
			}
		}
		most_scenic
	}

	fn scenary_score_for_tree(&self, row: usize, col: usize) -> i64 {
		let height = self.trees[row][col];

		let mut looking_west = 0;
		if col > 0 {
			for c in (0..col).rev() {
				let t = self.trees[row][c];
				looking_west += 1;
				if t >= height {
					break;
				}
			}
		}

		let mut looking_east = 0;
		if col < self.trees.cols() - 1 {
			for c in (col + 1)..self.trees.cols() {
				let t = self.trees[row][c];
				looking_east += 1;
				if t >= height {
					break;
				}
			}
		}

		let mut looking_south = 0;
		if row < self.trees.rows() - 1 {
			for r in (row + 1)..self.trees.rows() {
				let t = self.trees[r][col];
				looking_south += 1;
				if t >= height {
					break;
				}
			}
		}

		let mut looking_north = 0;
		if row > 0 {
			for r in (0..row).rev() {
				let t = self.trees[r][col];
				looking_north += 1;
				if t >= height {
					break;
				}
			}
		}

		looking_west * looking_east * looking_south * looking_north
	}
}

struct SumCalculator {}
impl Calculator<Record, Answer> for SumCalculator {
	fn solve_a(&mut self, records: &Vec<Record>) -> Result<Answer, String> {
		if let Ok(trees) = TreeGrid::new(&records.iter().collect()) {
			return Ok(trees.count_visible());
		} else {
			Err(format!("Error reading tree grid."))
		}
	}
	fn solve_b(&mut self, records: &Vec<Record>) -> Result<Answer, String> {
		if let Ok(trees) = TreeGrid::new(&records.iter().collect()) {
			return Ok(trees.most_scenic());
		} else {
			Err(format!("Error reading tree grid."))
		}
	}
}

fn main() {
	let mut parser = LineParser::new(&static_parse_line);
	let mut calculator = SumCalculator {};
	let status = Solution::solve::<Record, Answer>(&mut parser, &mut calculator);
	advent_exit(status);
}
