use std::env;
use std::fs;
use std::io::{BufRead, BufReader};
use std::process::exit;
use regex::Regex;

// SOME type
type Answer = (i64, i64);

#[derive(Clone)]
enum Data {
	NOP(i64),
	ACC(i64),
	JMP(i64),
}

fn evaluate(program: &Vec<Data>) -> (bool, i64) {
	let mut acc : i64 = 0;
	let mut ptr : i64 = 0;

	let mut visit_array : Vec<bool> = Vec::with_capacity(program.len());
	visit_array.resize_with(program.len(), || false);

	while (ptr as usize) < program.len() && !visit_array[ptr as usize] {
		visit_array[ptr as usize] = true;
		let instruction = &program[ptr as usize];
		match instruction {
			Data::NOP(_) => {
				ptr += 1
			},
			Data::ACC(n) => {
				acc += n;
				ptr += 1
			},
			Data::JMP(n) => {
				ptr += n
			}
		}
	}
	if (ptr as usize) >= program.len() {
		(true, acc)
	} else {
		(false, acc)
	}
}

fn calculate(data: &Vec<Data>) -> Result<Answer, &str> {
	let mut b : i64 = -1;

	// Part A
	let (_, a) = evaluate(&data);

	// Part B
	// Try swapping each JMP to NOP and vice versa
	// until we successfully complete the program.
	for (i, instruction) in data.iter().enumerate() {
		let mut program = data.to_vec();
		program[i] = match instruction {
			Data::NOP(n) => Data::JMP(*n),
			Data::JMP(n) => Data::NOP(*n),
			Data::ACC(n) => Data::ACC(*n),
		};
		let (completed, acc) = evaluate(&program);
		if completed {
			b = acc;
			break;
		}
	}
	
	Ok((a, b))
}

fn parse_line(line: &String) -> Data {
	let re = Regex::new(r"(jmp|acc|nop) ([-+]\d+)").unwrap();
	let cap = re.captures(line).unwrap();
	let number : i64 = cap.get(2).unwrap().as_str().parse::<i64>().unwrap();
	match cap.get(1).unwrap().as_str() {
		"jmp" => Data::JMP(number),
		"acc" => Data::ACC(number),
		"nop" => Data::NOP(0),
		_ => {
			println!("NOP");
			Data::NOP(0)
		}
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

