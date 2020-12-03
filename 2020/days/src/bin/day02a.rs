use std::env;
use std::fs;
use std::io::{BufRead, BufReader};
use std::process::exit;
use regex::Regex;

struct Data {
	min: i32,
	max: i32,
	letter: char,
	password: String,
}

fn parse_line(line: &String) -> Data {
	let re = Regex::new(r"(\d+)-(\d+) ([a-z]): ([a-z]+)").unwrap();
	let cap = re.captures(line).unwrap();
	let min : i32 = cap.get(1).unwrap().as_str().parse::<i32>().unwrap();
	let max : i32 = cap.get(2).unwrap().as_str().parse::<i32>().unwrap();
	let letter : char = cap.get(3).unwrap().as_str().parse::<char>().unwrap();
	let password : String = String::from(cap.get(4).unwrap().as_str());
	Data {
		min: min,
		max: max,
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

fn count_letters(letter: char, s: &str) -> i32 {
	let mut count = 0;
	for c in s.chars() {
		if c == letter {
			count += 1;
		}
	}
	count
}

fn calculate(data: &Vec<Data>) -> Option<i32> {
	let mut count = 0;
	for line in data {
		let check = count_letters(line.letter, line.password.as_str());
		if check >= line.min && check <= line.max {
			count += 1;
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

