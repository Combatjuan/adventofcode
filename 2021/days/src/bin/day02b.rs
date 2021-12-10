use std::env;
use std::fs;
use std::io::{BufRead, BufReader};
use std::process::exit;
use regex::Regex;

type Answer = i64;

enum Instruction {
	Forward(i64),
	Down(i64),
	Up(i64),
}

struct Data {
	instruction: Instruction
}

fn calculate(data: &Vec<Data>) -> Result<Answer, &str> {
	let mut horizontal : i64 = 0;
	let mut aim : i64 = 0;
	let mut depth : i64 = 0;
	for datum in data {
		match datum.instruction {
			Instruction::Forward(n) => {
				horizontal += n;
				depth += aim * n;
			},
			Instruction::Down(n) => aim += n,
			Instruction::Up(n) => aim -= n,
		}
	}
	Ok(horizontal * depth)
}

fn parse_line(line_number: usize, line: &String) -> Result<Data, String> {
	let re = Regex::new(r"(forward|down|up) (\d+)").unwrap();
	if let Some(cap) = re.captures(line) {
		let direction = cap.get(1).unwrap().as_str();
		let distance : i64 = cap.get(2).unwrap().as_str().parse::<i64>().unwrap();
		match direction {
			"forward" => Ok(Data { instruction : Instruction::Forward(distance) }),
			"down" => Ok(Data { instruction : Instruction::Down(distance) }),
			"up" => Ok(Data { instruction : Instruction::Up(distance) }),
			_ => Err(format!("Invalid direction: {}", direction))
		}
	} else {
		Err(format!("Failed to parse line {}: '{}'", line_number, line))
	} 
}

fn load(filename: &str) -> Result<Vec<Data>, String> {
	if let Ok(file) = fs::File::open(filename) {
		let lines = BufReader::new(file).lines();
		let data : Vec<Data> = lines.enumerate().map(
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
