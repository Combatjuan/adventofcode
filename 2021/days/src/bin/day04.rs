use std::env;
use std::fs;
use std::io::{BufRead, BufReader};
use std::process::exit;
use std::collections::HashMap;

const BOARD_SIZE : usize = 5;

type Answer = i64;

type Board = [ [(i64, bool); BOARD_SIZE]; BOARD_SIZE ];

trait Playable {
	fn find(&mut self, n: i64) -> bool;
	fn is_solved(&self) -> bool;
	fn print(&self);
	fn answer(&self, n: i64) -> Answer;
}

impl Playable for Board {
	fn find(&mut self, n: i64) -> bool {
		for row in self.iter_mut() {
			for (value, found) in row.iter_mut() {
				if *value == n {
					*found = true;
				}
			}
		}
		self.is_solved()
	}

	fn is_solved(&self) -> bool {
		for row in self {
			if row.iter().all(|&x| x.1) {
				return true;
			}
		}
		for col in 0..BOARD_SIZE {
			if self.iter().all(|&r| r[col].1) {
				return true;
			}
		}
		false
	}

	fn print(&self) {
		for row in self {
			for col in row {
				if col.1 {
					print!("[{}]\t", col.0);
				} else {
					print!(" {} \t", col.0);
				}
			}
			println!("");
		}
	}

	fn answer(&self, n: i64) -> Answer {
		let mut sum_unmatched : i64 = 0;
		for row in self {
			for col in row {
				if !col.1 {
					sum_unmatched += col.0;
				}
			}
		}
		println!("Unmatched Sum: {}", sum_unmatched);
		println!("Drawn: {}", n);
		sum_unmatched * n
	}
}

#[derive(Clone)]
struct Data {
	draws: Vec<i64>,
	boards: Vec<Board>,
}

fn calculate_a(data: &Data) -> Result<Answer, String> {
	println!("Part A:");
	println!("*******");
	let mut data_a = data.clone();
	for &n in data_a.draws.iter() {
		println!("Drew: {}", n);
		for board in data_a.boards.iter_mut() {
			if board.find(n) {
				board.print();
				return Ok(board.answer(n))
			}
			board.print();
			println!();
		}
	}
	Err("No board matched.".to_string())
}

fn calculate_b(data: &Data) -> Result<Answer, String> {
	println!("Part B:");
	println!("*******");
	let mut boards : Vec<Board> = data.boards.clone();

	// Keep going until we're out of boards
	for &n in data.draws.iter() {
		println!("Drew: {}", n);
		for board in boards.iter_mut() {
			if board.find(n) {
				println!("Solved:");
				board.print();
				println!();
			}
		}
		if boards.len() > 1 {
			boards.retain(|b| !b.is_solved());
		} else {
			let remaining = boards.first().unwrap();
			if remaining.is_solved() {
				println!("Last remaining board:");
				remaining.print();
				return Ok(remaining.answer(n))
			}
		}
	}
	Err("No board matched.".to_string())
}

fn calculate(data: &Data) -> (Result<Answer, String>, Result<Answer, String>) {
	match calculate_a(data) {
		Ok(a) => {
			match calculate_b(data) {
				Ok(b) => (Ok(a), Ok(b)),
				Err(b) => (Ok(a), Err(b)),
			}
		},
		Err(a) => (Err(a), Err(format!("First solve A")))
	}
}

fn str_to_numbers(s: &String) -> Vec<i64> {
	s.split_whitespace().map(|x| x.parse::<i64>().unwrap()).collect()
}

fn load(filename: &str) -> Result<Data, String> {
	if let Ok(file) = fs::File::open(filename) {
		let mut lines = BufReader::new(file).lines();
		let draws : Vec<i64> = lines.next().unwrap().unwrap().split(",").map(|x| x.parse::<i64>().unwrap()).collect();
		let mut boards : Vec<Board> = vec!();
		while let Some(Ok(line)) = lines.next() {
			if line.is_empty() {
				continue;
			} else {
				let row1 : Vec<i64> = str_to_numbers(&line);
				let row2 : Vec<i64> = str_to_numbers(&lines.next().unwrap().unwrap());
				let row3 : Vec<i64> = str_to_numbers(&lines.next().unwrap().unwrap());
				let row4 : Vec<i64> = str_to_numbers(&lines.next().unwrap().unwrap());
				let row5 : Vec<i64> = str_to_numbers(&lines.next().unwrap().unwrap());
				let board : Board = [
					[(row1[0], false), (row1[1], false), (row1[2], false), (row1[3], false), (row1[4], false)],
					[(row2[0], false), (row2[1], false), (row2[2], false), (row2[3], false), (row2[4], false)],
					[(row3[0], false), (row3[1], false), (row3[2], false), (row3[3], false), (row3[4], false)],
					[(row4[0], false), (row4[1], false), (row4[2], false), (row4[3], false), (row4[4], false)],
					[(row5[0], false), (row5[1], false), (row5[2], false), (row5[3], false), (row5[4], false)],
				];
				boards.push(board);
				board.print();
				println!();
			}
		}
		Ok(Data {
			draws,
			boards,
		})
	} else {
		Err(format!("Could not load data from '{}'.", filename))
	}
}

fn run(args: &Vec<String>) -> Result<(Answer, Answer), String> {
	if args.len() > 1 {
		let filename = &args[1];
		match load(filename) {
			Ok(data) => {
				match calculate(&data) {
					(Ok(a), Ok(b)) => {
						println!("Part A: {}", a);
						println!("Part B: {}", b);
						Ok((a, b))
					},
					(Ok(a), Err(b)) => {
						println!("Part A: {}", a);
						Err(b)
					},
					(Err(a), Err(_b)) => {
						Err(a)
					},
					(Err(a), Ok(b)) => {
						println!("Part B: {}", b);
						Err(a)
					}
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
		Ok((_, _)) => {
			println!("Two Stars!");
			0
		},
		Err(err) => {
			println!("{}", err);
			1
		}
	});
}

