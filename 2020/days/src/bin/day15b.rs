use std::env;
use std::fs;
use std::io::{BufRead, BufReader};
use std::process::exit;
use regex::Regex;

// For input 11,0,1,10,5,19
// Debug:
//	Takes 91 seconds without printing each line
//	Takes 186 seconds printing each line
// Release:
//  Takes 6 seconds without printing each line
//  Takes 77 seconds printing each line
type Answer = u32;

struct Data {
	numbers: Vec<u32>,
}

fn calculate(data: &Data, up_to: u32) -> Result<Answer, &str> {
	let mut mem : Vec<Option<u32>> = vec![];
	mem.resize((up_to + data.numbers.iter().max().unwrap() + 1) as usize, None);
	// Load starting values (except the last one because that's really
	// the first value we need to start crunching on.
	let stop_at = data.numbers.len() - 1;
	for (i, &n) in data.numbers.iter().enumerate() {
		if i == stop_at {
			break;
		}
		mem[n as usize] = Some((i + 1) as u32);
	}
	// Start with the last number
	let mut n : u32 = *data.numbers.last().unwrap();
	let mut i : u32 = (stop_at + 1) as u32;
	loop {
		// Recall if we've seen the number
		let next = if let Some(prev_index) = mem[n as usize] {
			//println!("Found {} last at {} so {} - {} = {}", n, prev_index, prev_index, i, i - prev_index);
			i - prev_index
		} else {
			0
		};
		mem[n as usize] = Some(i);
		n = next;

		// Keep going
		i += 1;
		if i == up_to {
			break;
		}
	};
	Ok(n as u32)
}

fn parse_line(line: &String) -> Data {
	let re = Regex::new(r"(\d+)").unwrap();
	let mut numbers : Vec<u32> = vec![];
	for m in re.find_iter(line) {
		numbers.push(m.as_str().parse::<u32>().unwrap());
	}
	Data {
		numbers
	}
}

fn load(filename: &str) -> Result<Data, String> {
	if let Ok(file) = fs::File::open(filename) {
		let mut lines = BufReader::new(file).lines();
		if let Ok(line) = lines.next().unwrap() {
			Ok(parse_line(&line))
		} else {
			Err(format!("Expected exactly one line in {}", filename))
		}
	} else {
		Err(format!("Could not load data from '{}'.", filename))
	}
}

fn run(args: &Vec<String>) -> Result<Answer, String> {
	if args.len() > 1 {
		let parsed_data : Result<Data, String> = if args[1].contains(',') {
			Ok(parse_line(&args[1]))
		} else {
			let filename = args[1].as_str();
			load(filename)
		};
		let nth = if args.len() > 2 {
			args[2].parse::<u32>().unwrap()
		} else {
			2020 as u32
		};
		match parsed_data {
			Ok(data) => {
				let answer = calculate(&data, nth);
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

