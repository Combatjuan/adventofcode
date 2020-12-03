use std::env;
use std::fs;
use std::io::{BufRead, BufReader};
use std::process::exit;
use regex::Regex;

struct Data {
	first: usize,
	second: usize,
	letter: char,
	password: String,
}

fn parse_line(line: &String) -> Data {
	let re = Regex::new(r"(\d+)-(\d+) ([a-z]): ([a-z]+)").unwrap();
	let cap = re.captures(line).unwrap();
	let first : usize = cap.get(1).unwrap().as_str().parse::<usize>().unwrap() - 1;
	let second : usize = cap.get(2).unwrap().as_str().parse::<usize>().unwrap() - 1;
	let letter : char = cap.get(3).unwrap().as_str().parse::<char>().unwrap();
	let password : String = String::from(cap.get(4).unwrap().as_str());
	Data {
		first: first,
		second: second,
		letter: letter,
		password: password,
	}
}

fn load(filename: &str) -> Vec<Data> {
	if let Ok(file) = fs::File::open(filename) {
		let lines = BufReader::new(file).lines();
		lines.map(|x| parse_line(&x.unwrap())).collect()
	} else {
		vec![]
	}
}

fn calculate(data: &Vec<Data>) -> Option<i32> {
	let mut count = 0;
	for line in data {
		let length = line.password.len();
		if line.first <= length && line.second <= length {
			let at_first = line.password.chars().nth(line.first).unwrap();
			let at_second = line.password.chars().nth(line.second).unwrap();
			if (at_first == line.letter) ^ (at_second == line.letter) {
				count += 1;
			}
		}
	}
	Some(count)
}

fn run(args: &Vec<String>) -> Result<i32, &'static str> {
	if args.len() > 1 {
		let filename = &args[1];
		let data = load(filename);
		let answer = calculate(&data);
		if let Some(a) = answer {
			Ok(a)
		} else {
			Err("Could not find a solution.")
		}
	} else {
		Err("Provide a file name please.")
	}
}

fn main() {
	let args: Vec<String> = env::args().collect();
	exit(match run(&args) {
		Ok(answer) => {
			println!("Ok Passwords: {}", answer);
			0
		},
		Err(err) => {
			println!("{}", err);
			1
		}
	});
}

