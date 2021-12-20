use std::env;
use std::fs;
use std::io::{BufRead, BufReader};
use std::process::exit;

// SOME type
type Answer = i64;

fn parse_line(line_number: usize, line: &String) -> Result<String, String> {
	if !line.is_empty() {
		Ok(line.to_string())
	} else {
		Err(format!("Empty line found on line {}", line_number))
	}
}

fn error_char_to_points(c: char) -> i64 {
	match c {
		')' => 3,
		']' => 57,
		'}' => 1197,
		'>' => 25137,
		_ => 0,
	}
}

fn first_error(s: &String) -> Option<char> {
	let mut stack : Vec<char> = vec!{};
	for c in s.chars() {
		match c {
			'(' => stack.push(')'),
			'[' => stack.push(']'),
			'{' => stack.push('}'),
			'<' => stack.push('>'),
			')'|']'|'}'|'>' => {
				if let Some(m) = stack.last() {
					if c == *m {
						stack.pop();
					} else {
						println!("Expected '{}' but instead found '{}'", *m, c);
						return Some(c);
					}
				} else {
					return Some(c);
				}
			}
			_ => {
				println!("Error: Unrecognized char '{}'", c);
				return Some('!');
			}
		}
	}
	None
}

fn calculate_a(data: &Vec<String>) -> Result<Answer, String> {
	let mut error_points : Answer = 0;
	for s in data {
		if let Some(c) = first_error(&s) {
			error_points += error_char_to_points(c);
		}
	}
	Ok(error_points)
}

fn missing_char_to_points(c: char) -> i64 {
	match c {
		')' => 1,
		']' => 2,
		'}' => 3,
		'>' => 4,
		_ => 0,
	}
}

fn remaining_code(s: &String) -> Vec<char> {
	let mut stack : Vec<char> = vec!{};
	for c in s.chars() {
		match c {
			'(' => stack.push(')'),
			'[' => stack.push(']'),
			'{' => stack.push('}'),
			'<' => stack.push('>'),
			')'|']'|'}'|'>' => {
				if let Some(m) = stack.last() {
					if *m == c {
						stack.pop();
					}
				}
			}
			_ => {
				panic!("Error: Unrecognized char '{}'", c);
			}
		}
	}
	stack.reverse();
	stack
}

fn calculate_b(data: &Vec<String>) -> Result<Answer, String> {
	let mut scores : Vec<Answer> = vec!{};
	for s in data {
		let mut score : Answer = 0;
		if let Some(c) = first_error(&s) {
			// Discard this line
		} else {
			let remaining = remaining_code(s);
			for c in remaining {
				score *= 5;
				score += missing_char_to_points(c);
			}
			scores.push(score);
		}
	}
	scores.sort();
	Ok(scores[scores.len() / 2])
}

fn calculate(data: &Vec<String>) -> (Result<Answer, String>, Result<Answer, String>) {
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

fn load(filename: &str) -> Result<Vec<String>, String> {
	if let Ok(file) = fs::File::open(filename) {
		let lines = BufReader::new(file).lines();
		let data : Vec<String> = lines.enumerate().map(
				|(n, x)|
				match parse_line(n + 1, &x.unwrap()) {
					Ok(data) => data,
					Err(msg) => panic!("{}", msg)
				}
			).collect();
		match data.is_empty() {
			true => Err(format!("Failed to parse every line in {}", filename)),
			false => Ok(data),
		}
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
