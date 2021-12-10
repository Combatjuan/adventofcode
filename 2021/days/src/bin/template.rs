use std::env;
use std::fs;
use std::io::{BufRead, BufReader};
use std::process::exit;
use regex::Regex;

// SOME type
type Answer = i64;

#[derive(Clone)]
struct Data {
    // Some type
}

fn parse_line(line_number: usize, line: &String) -> Result<Data, String> {
	let re = Regex::new(r"SOME regex").unwrap();
    if let Some(cap) = re.captures(line) {
		// SOME PARSING
        Ok(Data {
            42
        })
    } else {
        Err(format!("Failed to parse line {}: '{}'", line_number, line))
    } 
}

fn calculate_a(data: &Vec<Data>) -> Result<Answer, String> {
    Err(format!("Implement me"))
}

fn calculate_b(data: &Vec<Data>) -> Result<Answer, String> {
    Err(format!("Implement me"))
}

fn calculate(data: &Vec<Data>) -> Result<(Answer, Answer), String> {
	match calculate_a(data) {
		Ok(a) => {
			match calculate_b(data) {
				Ok(b) => Ok((a, b)),
				Err(b) => Err(b),
			}
		},
		Err(a) => Err(a)
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

fn run(args: &Vec<String>) -> Result<(Answer, Answer), String> {
	if args.len() > 1 {
		let filename = &args[1];
		match load(filename) {
			Ok(data) => {
				let answers = calculate(&data);
				if let Ok((a, b)) = answers {
					Ok((a, b))
				} else if let Err(s) = answers {
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
			println!("Part A: {}", a);
			println!("Part B: {}", b);
			0
		},
		Err(err) => {
			println!("{}", err);
			1
		}
	});
}

