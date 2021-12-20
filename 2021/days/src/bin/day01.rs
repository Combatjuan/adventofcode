use std::env;
use std::fs;
use std::io::{BufRead, BufReader};
use std::process::exit;
use regex::Regex;

// SOME type
type Answer = u64;

#[derive(Clone)]
struct Data {
	number: i64
}

fn summed(window_size: usize, data: &Vec<Data>) -> Vec<i64> {
	let count : usize = data.len();
	let mut sums : Vec<i64> = data.iter().take(count - window_size + 1).map(|o| o.number).collect();
	for i in 1..(window_size as usize) {
		for n in 0..(count - window_size + 1) {
			sums[n] += data[n + i].number;
		}
	}
	sums
}

fn calculate(window_size: usize, data: &Vec<Data>) -> Result<Answer, &str> {
	let mut increase_counter : u64 = 0;
	let sums = summed(window_size, &data);
	let mut sum_iterator = sums.iter();
	let mut last : i64 = *sum_iterator.next().unwrap();
	for n in sum_iterator {
		if *n > last {
			increase_counter += 1;
		}
		last = *n;
	}
	Ok(increase_counter)
}

fn parse_line(line_number: usize, line: &String) -> Result<Data, String> {
	let re = Regex::new(r"\d+").unwrap();
	if let Some(cap) = re.captures(line) {
		let number : i64 = cap.get(0).unwrap().as_str().parse::<i64>().unwrap();
		Ok(Data {
			number
		})
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
		let window_size : usize = if args.len() > 2 { args[2].parse::<usize>().unwrap() } else { 1 };
		match load(filename) {
			Ok(data) => {
				let answer = calculate(window_size, &data);
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

