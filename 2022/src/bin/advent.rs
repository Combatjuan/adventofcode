use std::env;
use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::exit;

pub struct Status {
	status: Result<(), String>,
	answer_a: Result<String, String>,
	answer_b: Result<String, String>,
}

impl Status {
	fn from_err(err: String) -> Status {
		Status {
			status: Err(err),
			answer_a: Ok("".to_string()),
			answer_b: Ok("".to_string()),
		}
	}
	fn from_a_b(answer_a: Result<String, String>, answer_b: Result<String, String>) -> Status {
		Status {
			status: Ok(()),
			answer_a,
			answer_b,
		}
	}
}

pub trait Parser<Record> {
	fn parse(&mut self, file: File) -> Result<Vec<Record>, String>;
}

pub struct LineParser<Record> {
	line_parser: Box<dyn Fn(usize, &String) -> Result<Record, String>>
}

impl<Record> LineParser<Record> {
	#[allow(dead_code)]
	pub fn new(f: &'static dyn Fn(usize, &String) -> Result<Record, String>) -> LineParser<Record> {
		LineParser {
			line_parser: Box::new(f)
		}
	}
}

impl<Record> Parser<Record> for LineParser<Record> {
	fn parse(&mut self, file: File) -> Result<Vec<Record>, String> {
		let lines = BufReader::new(file).lines();
		let records : Vec<Record> = lines.enumerate().map(
				|(n, x)|
				match (self.line_parser)(n + 1, &x.unwrap()) {
					Ok(records) => records,
					Err(msg) => panic!("{}", msg)
				}
			).collect();
		match records.is_empty() {
			true => Err(format!("Failed to parse every line in file.")),
			false => Ok(records),
		}
	}
}

/// StringVecParser
/// This class just converts our input file into a vector of strings which
/// means we don't need to have BufReaders and Files imported into every
/// day's solution.
pub struct StringVecParser<Record> {
	parser: Box<dyn Fn(Vec<String>) -> Result<Vec<Record>, String>>
}

impl<Record> StringVecParser<Record> {
	#[allow(dead_code)]
	pub fn new(f: &'static dyn Fn(Vec<String>) -> Result<Vec<Record>, String>) -> StringVecParser<Record> {
		StringVecParser {
			parser: Box::new(f)
		}
	}
}

impl<Record> Parser<Record> for StringVecParser<Record> {
	fn parse(&mut self, file: File) -> Result<Vec<Record>, String> {
		let lines = BufReader::new(file).lines().map(|x| x.unwrap()).collect();
		let records = (self.parser)(lines);
		match records {
			Ok(records) => {
				match records.is_empty() {
					true => Err(format!("Failed to parse every line in file.")),
					false => Ok(records),
				}
			},
			Err(msg) => Err(msg)
		}
	}
}

pub trait Calculator<Record, Answer> {
	fn solve_a(&mut self, records: &Vec<Record>) -> Result<Answer, String>;
	fn solve_b(&mut self, records: &Vec<Record>) -> Result<Answer, String>;
}

/*
impl<Record, Answer> dyn Calculator<Record, Answer> {
	fn solve_a(&mut self, records: &Vec<Record>) -> Result<Answer, String> {
		Err("Unimplemented".to_string())
	}
	fn solve_b(&mut self, records: &Vec<Record>) -> Result<Answer, String> {
		Err("Unimplemented".to_string())
	}
}
*/

pub struct Solution {}

impl Solution {
	pub fn solve<Record, Answer>(
			parser: &mut dyn Parser<Record>,
			calculator: &mut dyn Calculator<Record, Answer>
	) -> Status
	where Answer: Display 
	{
		let args: Vec<String> = env::args().collect();
		if args.len() > 1 {
			let filename = &args[1];
			if let Ok(file) = File::open(filename) {
				let records = parser.parse(file);
				match records {
					Ok(records) => {
						let answer_a = calculator.solve_a(&records);
						let display_a = match answer_a {
							Ok(a) => Ok(format!("Part A Answer: {}", a)),
							Err(msg) => Err(format!("Part A Failed: {} ", msg))
						};
						let answer_b = calculator.solve_b(&records);
						let display_b = match answer_b {
							Ok(a) => Ok(format!("Part B Answer: {}", a)),
							Err(msg) => Err(format!("Part B Failed: {} ", msg))
						};
						Status::from_a_b(display_a, display_b)
					},
					Err(msg) => Status::from_err(format!("Parsing: Could not parse file: '{}' {}", filename, msg))
				}
			} else {
				Status::from_err(format!("File: Failed to open file '{}'", filename))
			}
		} else {
			Status::from_err(format!("Usage: Incorrect arguments specified"))
		}
	}
}
 
pub fn advent_exit(status: Status) {
	exit(match status {
		Status { status: Err(e), answer_a: _, answer_b: _ } => {
			println!("Error: {}", e);
			1
		},
		Status { status: _, answer_a: Ok(a), answer_b: Ok(b) } => {
			println!("{}", a);
			println!("{}", b);
			0
		},
		Status { status: _, answer_a: Ok(a), answer_b: Err(e) } => {
			println!("{}", a);
			println!("{}", e);
			2
		},
		Status { status: _, answer_a: Err(e), answer_b: Ok(b) } => {
			println!("{}", e);
			println!("{}", b);
			3
		},
		Status { status: _, answer_a: Err(a), answer_b: Err(b) } => {
			println!("{}", a);
			println!("{}", b);
			4
		},
	});
}

#[allow(dead_code)]
fn main() {
	println!("Please use --bin to specify the correct day to run.");
}
