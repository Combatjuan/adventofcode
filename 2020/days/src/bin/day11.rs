use std::fmt;
use std::fmt::Display;
use std::env;
use std::fs;
use std::io::{BufRead, BufReader};
use std::process::exit;

// SOME type
type Answer = (i64, i64);

struct Data {
	line: String,
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Spot {
	Floor,
	Empty,
	Occupied,
}

impl Display for Spot {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", match self {
			Spot::Floor => '.',
			Spot::Empty => 'L',
			Spot::Occupied => '#',
		})
	}
}

struct Seats {
	seats: Vec<Vec<Spot>>,
	rows: usize,
	cols: usize,
	changed: bool
}

const OFFSETS : [Option<(i32, i32)>; 9] = [
	Some((-1, -1)), Some((-1, 0)), Some((-1, 1)), Some((0, -1)), Some((0, 1)), Some((1, -1)), Some((1, 0)), Some((1, 1)), None
];

struct SpotIteratorA<'a> {
	seats: &'a Vec<Vec<Spot>>,
	row: usize,
	col: usize,
	rows: i32,
	cols: i32,
	offset_index: i32,
}

impl <'a>Iterator for SpotIteratorA<'a> {
	type Item = Spot;
	fn next(&mut self) -> Option<Spot> {
		if self.offset_index == 8 {
			None
		} else {
			while self.offset_index <= 8 {
				self.offset_index += 1;
				if self.offset_index == 8 {
					return None;
				} else {
					let next_spot : (i32, i32) = (
						self.row as i32 + OFFSETS[self.offset_index as usize].unwrap().0,
						self.col as i32 + OFFSETS[self.offset_index as usize].unwrap().1,
					);
					if next_spot.0 >= 0 && next_spot.1 >= 0
						&& next_spot.0 < self.rows && next_spot.1 < self.cols {
						return Some(self.seats[next_spot.0 as usize][next_spot.1 as usize]);
					}
				}
			}
			None
		}
	}
}

struct SpotIteratorB<'a> {
	seats: &'a Vec<Vec<Spot>>,
	row: usize,
	col: usize,
	rows: i32,
	cols: i32,
	offset_index: i32,
}

impl <'a> SpotIteratorB<'_> {
	fn in_direction(&self, dx: i32, dy: i32) -> Option<Spot> {
		let r = self.row as i32;
		let c = self.col as i32;
		let mut n = 1;
		loop {
			let dr = r + dx * n;
			let dc = c + dy * n;
			if dr < 0 || dc < 0 || dr >= self.rows || dc >= self.cols {
				break None;
			} else if self.seats[dr as usize][dc as usize] == Spot::Floor {
				n += 1;
			} else {
				break Some(self.seats[dr as usize][dc as usize]);
			}
		}
	}
}

impl <'a>Iterator for SpotIteratorB<'a> {
	type Item = Spot;
	fn next(&mut self) -> Option<Spot> {
		if self.offset_index == 8 {
			None
		} else {
			while self.offset_index <= 8 {
				self.offset_index += 1;
				if self.offset_index == 8 {
					return None;
				} else if let Some(spot) = self.in_direction(
						OFFSETS[self.offset_index as usize].unwrap().0,
						OFFSETS[self.offset_index as usize].unwrap().1
					) {
					return Some(spot);
				}
			}
			None
		}
	}
}

impl Seats {
	fn new(data: &Vec<Data>) -> Seats {
		let mut seats : Vec<Vec<Spot>> = vec![];
		let rows = data.len();
		let cols = if data.is_empty() { 0 } else { data.first().unwrap().line.len() };
		for d in data {
			let mut row = Vec::<Spot>::new();
			for c in d.line.chars() {
				row.push(match c {
					'#' => Spot::Occupied,
					'.' => Spot::Floor,
					'L' => Spot::Empty,
					_ => Spot::Floor,
				})
			}
			assert!(row.len() == cols);
			seats.push(row);
		}
		Seats {
			seats,
			rows: rows,
			cols: cols,
			changed: true,
		}
	}

	fn at(&self, row: usize, col: usize) -> Spot {
		self.seats[row][col]
	}

	fn adjacents(&self, row: usize, col: usize) -> SpotIteratorA {
		SpotIteratorA {
			seats: &self.seats,
			row: row,
			col: col,
			rows: self.rows as i32,
			cols: self.cols as i32,
			offset_index: -1,
		}
	}

	fn visible_from(&self, row: usize, col: usize) -> SpotIteratorB {
		SpotIteratorB {
			seats: &self.seats,
			row: row,
			col: col,
			rows: self.rows as i32,
			cols: self.cols as i32,
			offset_index: -1,
		}
	}

	fn occupied(&self) -> i64 {
		let mut count = 0;
		for row in &self.seats {
			for col in row {
				if *col == Spot::Occupied {
					count += 1;
				}
			}
		}
		count
	}

	fn next_a(&self) -> Seats {
		let mut seats : Vec<Vec<Spot>> =
			(0..self.rows).map(|_| (0..self.cols).map(|_| Spot::Floor).collect()).collect();

		let mut changed = false;
		for (r, row) in self.seats.iter().enumerate() {
			for (c, _) in row.iter().enumerate() {
				seats[r][c] = match self.at(r, c) {
					Spot::Floor => Spot::Floor,
					Spot::Occupied => {
						if self.adjacents(r, c).filter(|x| *x == Spot::Occupied).count() >= 4 {
							changed = true;
							Spot::Empty
						} else {
							Spot::Occupied
						}
					},
					Spot::Empty => {
						if self.adjacents(r, c).filter(|x| *x == Spot::Occupied).count() == 0 {
							changed = true;
							Spot::Occupied
						} else {
							Spot::Empty
						}
					}
				};
			}
		}

		Seats {
			seats,
			rows: self.rows,
			cols: self.cols,
			changed,
		}
	}

	fn next_b(&self) -> Seats {
		let mut seats : Vec<Vec<Spot>> =
			(0..self.rows).map(|_| (0..self.cols).map(|_| Spot::Floor).collect()).collect();

		let mut changed = false;
		for (r, row) in self.seats.iter().enumerate() {
			for (c, _) in row.iter().enumerate() {
				seats[r][c] = match self.at(r, c) {
					Spot::Floor => Spot::Floor,
					Spot::Occupied => {
						if self.visible_from(r, c).filter(|x| *x == Spot::Occupied).count() >= 5 {
							changed = true;
							Spot::Empty
						} else {
							Spot::Occupied
						}
					},
					Spot::Empty => {
						if self.visible_from(r, c).filter(|x| *x == Spot::Occupied).count() == 0 {
							changed = true;
							Spot::Occupied
						} else {
							Spot::Empty
						}
					}
				};
			}
		}

		Seats {
			seats,
			rows: self.rows,
			cols: self.cols,
			changed,
		}
	}
}

impl Display for Seats {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		for row in &self.seats {
			for col in row {
				if let Err(s) = write!(f, "{}", col) {
					return Err(s);
				}
			}
			if let Err(s) = write!(f, "\n") {
				return Err(s);
			}
		}
		Ok(())
	}
}

fn calculate(data: &Vec<Data>) -> Result<Answer, &str> {
	let mut seats_a = Seats::new(data);

	// Part A
	let mut iterations = 0;
	while seats_a.changed {
		println!("=== {} ===", iterations);
		println!("{}\n", seats_a);
		seats_a = seats_a.next_a();
		iterations += 1;
	}

	// Part B
	iterations = 0;
	let mut seats_b = Seats::new(data);
	while seats_b.changed {
		println!("=== {} ===", iterations);
		println!("{}\n", seats_b);
		seats_b = seats_b.next_b();
		iterations += 1;
	}
	Ok((seats_a.occupied(), seats_b.occupied()))
}

fn parse_line(line: &String) -> Data {
	Data {
		line: line.clone()
	}
}

fn load(filename: &str) -> Result<Vec<Data>, String> {
	if let Ok(file) = fs::File::open(filename) {
		let lines = BufReader::new(file).lines();
		let data : Vec<Data> = lines.map(|x| parse_line(&x.unwrap())).collect();
		match data.is_empty() {
			true => Err(format!("Failed to parse every line in {}", filename)),
			false => Ok(data),
		}
	} else {
		Err(format!("Could not load data from '{}'.", filename))
	}
}

fn run(args: &Vec<String>) -> Result<Answer, String> {
	if args.len() > 1 {
		let filename = &args[1];
		match load(filename) {
			Ok(data) => {
				let answer = calculate(&data);
				if let Ok(a) = answer {
					Ok(a)
				} else if let Err(s) = answer {
					Err(format!("Could not find a solution: {} ", s))
				} else {
					Err(String::from("This branch is not reachable"))
				}
			},
			Err(msg) => Err(msg)
		}
	} else {
		Err(String::from("Provide a file name please."))
	}
}

fn main() {
	let args: Vec<String> = env::args().collect();
	exit(match run(&args) {
		Ok((a, b)) => {
			println!("Answer A: {}", a);
			println!("Answer B: {}", b);
			0
		},
		Err(err) => {
			println!("{}", err);
			1
		}
	});
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn construct_seats() {
		let data : Vec<Data> = vec![
			Data{line:"###".to_string()},
			Data{line:"###".to_string()},
			Data{line:"###".to_string()},
		];
		let seats = Seats::new(&data);
		assert_eq!(seats.rows, 3);
	}

	#[test]
	fn count_adjacents() {
		let data : Vec<Data> = vec![
			Data{line:"LLL".to_string()},
			Data{line:"L#.".to_string()},
			Data{line:"...".to_string()},
		];
		let seats = Seats::new(&data);
		let adj : Vec<Spot> = seats.adjacents(1, 1).collect();
		println!("{:?}", adj);
		assert_eq!(seats.adjacents(1, 1).count(), 8);
	}

	#[test]
	fn count_visible_from() {
		let data : Vec<Data> = vec![
			Data{line:"..#..".to_string()},
			Data{line:".....".to_string()},
			Data{line:"..##.".to_string()},
			Data{line:".....".to_string()},
			Data{line:"#...#".to_string()},
		];
		let seats = Seats::new(&data);
		let adj : Vec<Spot> = seats.visible_from(2, 2).collect();
		println!("{:?}", adj);
		assert_eq!(seats.visible_from(2, 2).count(), 4);
	}
}
