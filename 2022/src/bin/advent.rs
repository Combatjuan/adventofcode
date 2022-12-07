use std::env;
use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub trait Parser<Record> {
	fn parse(&mut self, file: File) -> Result<Vec<Record>, String>;
}

pub struct LineParser<Record> {
	line_parser: Box<dyn Fn(usize, &String) -> Result<Record, String>>
}

impl<Record> LineParser<Record> {
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

pub trait Calculator<Record, Answer> {
	fn solve(&mut self, records: &Vec<Record>) -> Result<Answer, String>;
}

pub struct Solution {}

//impl Solution<Parser<Record>, Calculator<Record, Answer>> {
impl Solution {
	pub fn solve<Record, Answer>(
			parser: &mut dyn Parser<Record>,
			calculator: &mut dyn Calculator<Record, Answer>
	) -> Result<String, String>
	where Answer: Display 
	{
		let args: Vec<String> = env::args().collect();
		if args.len() > 1 {
			let filename = &args[1];
			if let Ok(file) = File::open(filename) {
				let records = parser.parse(file);
				match records {
					Ok(records) => {
						let answer = calculator.solve(&records);
						match answer {
							Ok(a) => Ok(format!("{}", a)),
							Err(msg) => Err(format!("Could not find a solution: {} ", msg))
						}
					},
					Err(msg) => Err(format!("Could not parse file: {} ", msg))
				}
			} else {
				Err(String::from("Provide a file name please."))
			}
		} else {
			Err(format!("Incorrect arguments specified"))
		}
	}
}
 
/*
// -----------------------------------------------------------------------------
// -----------------------------------------------------------------------------
// -----------------------------------------------------------------------------
struct SumCalculator {}
impl Calculator<i64, i64> for SumCalculator {
	fn solve(&mut self, records: &Vec<i64>) -> Result<i64, String> {
		let mut n : i64 = 0;
		for r in records {
			n += r;
		}
		Ok(n)
	}
}

fn static_parse_line(line_number: usize, s: &String) -> Result<i64, String> {
	let re = Regex::new(r"\d+").unwrap();
	if let Some(cap) = re.captures(s) {
		let number : i64 = cap.get(0).unwrap().as_str().parse::<i64>().unwrap();
		Ok(number)
	} else {
		Err(format!("Failed to parse line {}: '{}'", line_number, s))
	} 
}

fn main() {
	let mut parser = LineParser {line_parser: Box::new(static_parse_line)};
	let mut calculator = SumCalculator {};
	exit(match Solution::solve::<i64, i64>(&mut parser, &mut calculator) {
		Ok(answer) => {
			println!("Answer: {}", answer);
			0
		},
		Err(err) => {
			println!("{}", err);
			1
		}
	});
}
*/
