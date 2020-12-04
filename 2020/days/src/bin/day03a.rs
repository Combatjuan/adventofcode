use std::env;
use std::fs;
use std::io::{BufRead, BufReader};
use std::process::exit;
use regex::Regex;

// SOME type
type Answer = i64;

struct Data {
	trees: Vec<i32>,
	length: usize,
}

fn parse_line(line: &String) -> Data {
	let re = Regex::new(r"#").unwrap();
	let cap = re.find_iter(line);
	let mut trees = vec![];
	for m in cap {
		trees.push(m.start() as i32);
	}
	Data {
		trees: trees,
		length: line.len(),
	}
}

fn calculate(data: &Vec<Data>) -> Result<Answer, &str> {
	const SLOPE : i32 = 3;
	let mut x : i32 = 0;
	let mut trees = 0;
	for row in data {
		let mut found = false;
		for tree in &row.trees {
			let relative_x = x % (row.length as i32);
			if *tree == relative_x {
				found = true;
				continue
			}
		}
		if found {
			trees += 1;
		}
		x += SLOPE;
	}
	Ok(trees)
}


fn load(filename: &str) -> Vec<Data> {
	if let Ok(file) = fs::File::open(filename) {
		let lines = BufReader::new(file).lines();
		lines.map(|x| parse_line(&x.unwrap())).collect()
	} else {
		vec![]
	}
}

fn run(args: &Vec<String>) -> Result<Answer, String> {
	if args.len() > 1 {
		let filename = &args[1];
		let data = load(filename);
		let answer = calculate(&data);
		if let Ok(a) = answer {
			Ok(a)
		} else if let Err(s) = answer {
			Err(format!("Could not find a solution: {} ", s))
		} else {
			Err(String::from("This branch is not reachable"))
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

