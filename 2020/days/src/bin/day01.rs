use std::env;
use std::fs;
use std::io::{BufRead, BufReader};
use std::process::exit;

const SUM_TO : i64 = 2020;

struct Data {
	number: i64,
}

fn parse_line(line: &String) -> Data {
	if let Ok(number) = line.parse::<i64>() {
		Data { number }
	} else {
		println!("Parse error for line: '{}'", line);
		Data { number: 0}
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

fn calculate(data: &Vec<Data>) -> Option<(i64, i64)> {
	let mut two_numbers : Option<i64> = None;
	let mut three_numbers : Option<i64> = None;
	for Data{number: n} in data {
		if n <= &SUM_TO {
			for Data{number: m} in data {
				if n + m == SUM_TO {
					println!("{} + {} = {}", n, m, SUM_TO);
					two_numbers = Some(n * m);
				} else if n + m < SUM_TO {
					for Data{number: o} in data {
						if n + m + o == SUM_TO {
							println!("{} + {} + {} = {}", n, m, o, SUM_TO);
							three_numbers = Some(n * m * o);
						}
					}
				}
			}
		}
	}

	if two_numbers == None {
		println!("Could not find solution for two numbers.");
		None
	} else if three_numbers == None {
		println!("Could not find solution for three numbers.");
		None
	} else {
		Some((two_numbers.unwrap(), three_numbers.unwrap()))
	}
}

fn run(args: &Vec<String>) -> Result<(i64, i64), &'static str> {
	if args.len() > 1 {
		let filename = &args[1];
		let data = load(filename);
		let answer = calculate(&data);
		if let Some((a, b)) = answer {
			Ok((a, b))
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
		Ok((two, three)) => {
			println!("Two Items: {}", two);
			println!("Three Items: {}", three);
			0
		},
		Err(err) => {
			println!("{}", err);
			1
		}
	});
}

