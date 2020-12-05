use std::env;
use std::fs;
use std::io::{BufRead, BufReader};
use std::process::exit;

// SOME type
type Answer = (i32, i32);

struct Data {
	id: i32,
}

fn find_missing(data: &Vec<Data>) -> i32 {
	let mut ids : Vec<i32> = data.iter().map(|x| x.id).collect();
	ids.sort();
	assert!(ids.len() > 1);
	let mut missing_id = -1;

	for i in 1..ids.len() - 2 {
		if ids[i] - ids[i-1] != 1 {
			missing_id = ids[i] - 1;
			println!("Missing: {}", missing_id);
		}
	}
	missing_id
}

fn calculate(data: &Vec<Data>) -> Result<Answer, &str> {
	let min_item : Data = Data {
		id: 0,
	};
	let mut max_item : &Data = &min_item;
	for item in data {
		if item.id > max_item.id {
			max_item = item;
		}
	}
	let ticket = find_missing(data);
	Ok((max_item.id, ticket))
	//Ok(data.iter().map(|d| d.id).max().unwrap())
}

fn expand_code(line: &String) -> i32 {
	let mut row: i32 = 0;
	let mut col: i32 = 0;
	for (i, c) in line.chars().enumerate().take(7) {
		let add : i32 = 1 << (6 - i as i32);
		if c == 'B' {
			row += add;
		}
	}
	for (i, c) in line.chars().skip(7).enumerate().take(3) {
		let add : i32 = 1 << (2 - i as i32);
		if c == 'R' {
			col += add;
		}
	}
	row * 8 + col
}

fn parse_line(line: &String) -> Data {
	let id = expand_code(line);
	Data {
		id: id,
	}
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
		Ok((a, b)) => {
			println!("Answer A: {}", a);
			println!("Answer B: {}", b);
			0
		},
		Err(err) => {
			println!("{}", err);
			1
		}
	});
}

