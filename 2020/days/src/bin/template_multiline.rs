use std::env;
use std::fs;
use std::io::{BufRead, BufReader};
use std::process::exit;
use regex::Regex;

// SOME type
type Answer = i64;

struct Data {
	// SOME data
}

fn parse(lines: &mut dyn Iterator<Item=Result<String, std::io::Error>>) -> Option<Data> {
	let _re = Regex::new(r"SOME REGEX").unwrap();
	loop {
		let next_line = lines.next();
		let _line = match next_line {
			None => { break },
			Some(Ok(l)) if l.is_empty() => { break },
			Some(Err(_)) => { break },
			Some(Ok(l)) => l,
		};
	}
	Some(Data {})
}

fn calculate(_data: &Vec<Data>) -> Result<Answer, &str> {
	// SOME calculation
	Err("Implement me")
}

fn load(filename: &str) -> Result<Vec<Data>, String> {
	if let Ok(file) = fs::File::open(filename) {
		let mut lines : &mut dyn Iterator<Item=Result<String, std::io::Error>> = &mut BufReader::new(file).lines();
		let mut data : Vec<Data> = vec![];
		while let Some(record) = parse(&mut lines) {
			data.push(record);
		}
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

