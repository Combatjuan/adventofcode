use std::env;
use std::fs;
use std::io::{BufRead, BufReader};
use std::process::exit;

// SOME type
type Answer = u64;

#[derive(Clone)]
struct Data {
	crabs: Vec<i32>
}

fn str_to_numbers(s: &String) -> Vec<i32> {
	s.split(",").map(|x| x.parse::<i32>().unwrap()).collect()
}

fn parse_line(line: &String) -> Result<Data, String> {
	Ok(Data {
		crabs: str_to_numbers(line)
	})
}

fn calculate_a(data: &Data) -> Result<Answer, String> {
	Err("Implement me".to_string())
}

fn calculate_b(data: &Data) -> Result<Answer, String> {
	Err("Implement me".to_string())
}

fn calculate(data: &Data) -> (Result<Answer, String>, Result<Answer, String>) {
	match calculate_a(data) {
		Ok(a) => {
			match calculate_b(data) {
				Ok(b) => (Ok(a), Ok(b)),
				Err(b) => (Ok(a), Err(b)),
			}
		},
		Err(a) => (Err(a), Err(format!("First solve A")))
	}
}

fn load(filename: &str) -> Result<Data, String> {
	if let Ok(file) = fs::File::open(filename) {
		let mut lines = BufReader::new(file).lines();
		let line = lines.next().unwrap().unwrap();
		parse_line(&line)
	} else {
		Err(format!("Could not load data from '{}'.", filename))
	}
}

fn run(args: &Vec<String>) -> Result<(Answer, Answer), String> {
	if args.len() > 1 {
		let filename = &args[1];
		match load(filename) {
			Ok(data) => {
				match calculate(&data) {
					(Ok(a), Ok(b)) => {
						println!("Part A: {}", a);
						println!("Part B: {}", b);
						Ok((a, b))
					},
					(Ok(a), Err(b)) => {
						println!("Part A: {}", a);
						Err(b)
					},
					(Err(a), Err(_b)) => {
						Err(a)
					},
					(Err(a), Ok(b)) => {
						println!("Part B: {}", b);
						Err(a)
					}
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
		Ok((_, _)) => {
			println!("Two Stars!");
			0
		},
		Err(err) => {
			println!("{}", err);
			1
		}
	});
}
