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

fn parse_line(line: &String) -> Data {
	let re = Regex::new(r"SOME regex").unwrap();
	let cap = re.captures(line).unwrap();
		// SOME parsing
	Data {
			// SOME Data
	}
}

fn calculate(data: &Vec<Data>) -> Result<Answer, &str> {
	// SOME calculation
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

