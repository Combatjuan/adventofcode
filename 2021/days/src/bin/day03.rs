use std::env;
use std::fs;
use std::io::{BufRead, BufReader};
use std::process::exit;

// SOME type
type Answer = i64;

#[derive(Clone)]
struct Data {
	bits : Vec<bool>
}

fn parse_line(line_number: usize, line: &String) -> Result<Data, String> {

	let mut bits : Vec<bool> = vec!{};

	for c in line.chars() {
		match c {
			'0' => bits.push(false),
			'1' => bits.push(true),
			_ => {
				return Err(format!("Line {}: '{}' had invalid bits '{}'", line_number, line, c));
			}
		}
	}
	Ok(Data{bits})
}

fn most_common_bit_iter<'a, I>(data_iter: I, index: usize, default: bool) -> bool
    where
        I: std::iter::ExactSizeIterator<Item = &'a Data>
{
    let count = data_iter.len();
    let true_count : usize = data_iter.filter(|x| x.bits[index]).count();
    if true_count > (count - true_count) {
        true
    } else if true_count < (count - true_count) {
        false
    } else {
        default
    }
}

fn bool_vec_to_number(v: &Vec<bool>) -> i64 {
    let mut s = String::new();
    for b in v {
        match b {
            false => s.push('0'),
            true => s.push('1'),
        }
    }
    i64::from_str_radix(&s, 2).unwrap()
}

fn calculate_a(data: &Vec<Data>) -> Result<Answer, String> {
	let bit_length = data[0].bits.len();
	let mut gamma = String::new();
	let mut epsilon = String::new();
	for i in 0..bit_length {
        let mcb = most_common_bit_iter(data.iter(), i, true);
		if mcb {
			gamma.push('1');
			epsilon.push('0');
		} else {
			gamma.push('0');
			epsilon.push('1');
		}
    }
	let gamma = i64::from_str_radix(&gamma, 2).unwrap(); 
	let epsilon = i64::from_str_radix(&epsilon, 2).unwrap(); 
	Ok(gamma * epsilon)
}

fn calculate_b(data: &Vec<Data>) -> Result<Answer, String> {
    let length = data[0].bits.len();

    // Calculate Oxygen
    let mut remaining : Vec<Data> = data.to_vec();
    for index in 0..length {
        let bit_to_match = most_common_bit_iter(remaining.iter(), index, true);
        remaining = remaining.iter().filter(|x| x.bits[index] == bit_to_match).cloned().collect();
        if remaining.len() == 1 { break }
    }
    if remaining.len() != 1 {
        return Err(format!("Oxygen generator found no solution"));
    }
    let oxygen_generator = bool_vec_to_number(&remaining[0].bits);

    let mut remaining : Vec<Data> = data.to_vec();
    for index in 0..length {
        let bit_to_match = most_common_bit_iter(remaining.iter(), index, true);
        remaining = remaining.iter().filter(|x| x.bits[index] != bit_to_match).cloned().collect();
        if remaining.len() == 1 { break }
    }
    if remaining.len() != 1 {
        return Err(format!("CO2 scrubber found no solution"));
    }
    let co2_scrubber = bool_vec_to_number(&remaining[0].bits);
    Ok(oxygen_generator * co2_scrubber) 
}

fn calculate(data: &Vec<Data>) -> Result<(Answer, Answer), String> {
	match calculate_a(data) {
		Ok(a) => {
			match calculate_b(data) {
				Ok(b) => Ok((a, b)),
				Err(b) => Err(b),
			}
		},
		Err(a) => Err(a)
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

fn run(args: &Vec<String>) -> Result<(Answer, Answer), String> {
	if args.len() > 1 {
		let filename = &args[1];
		match load(filename) {
			Ok(data) => {
				let answers = calculate(&data);
				if let Ok((a, b)) = answers {
					Ok((a, b))
				} else if let Err(s) = answers {
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
			println!("Part A: {}", a);
			println!("Part B: {}", b);
			0
		},
		Err(err) => {
			println!("{}", err);
			1
		}
	});
}

