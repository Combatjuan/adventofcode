use std::env;
use std::fs;
use std::collections::HashSet;
use std::io::{BufRead, BufReader};
use std::process::exit;

// SOME type
type Answer = i32;

struct Data {
	questions: HashSet<char>
}

fn parse(lines: &mut dyn Iterator<Item=Result<String, std::io::Error>>) -> Option<Data> {
	let mut questions : HashSet<char> = HashSet::new();
	loop {
		let next_line = lines.next();
		let line = match next_line {
			None => { break },
			Some(Ok(l)) if l.is_empty() => { break },
			Some(Err(_)) => { break },
			Some(Ok(l)) => l,
		};
		for c in line.chars() {
			questions.insert(c);
		}
	}
	if questions.is_empty() {
		None
	} else {
		Some(Data {
			questions: questions
		})
	}
}

fn calculate(data: &Vec<Data>) -> Result<Answer, &str> {
	let mut sum : Answer = 0;
	for group in data {
		let count = group.questions.len() as i32;
		println!("{} <- {}", count, group.questions.iter().collect::<String>());
		sum += count;
	}
	Ok(sum)
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

