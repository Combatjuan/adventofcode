use std::collections::{BTreeMap, HashMap};
use std::env;
use std::fs;
use std::io::{BufRead, BufReader};
use std::process::exit;
use regex::Regex;

// SOME type
type Answer = (u32, u64);

#[derive(Debug)]
struct Data {
	fields: Vec<Vec<(u32, u32)>>,
	field_names: BTreeMap<usize, String>,
	mine: Vec<u32>,
	nearby: Vec<Vec<u32>>,
}

enum ParseState {
	Fields,
	Mine,
	Nearby
}

fn ranges_contains(ranges: &Vec<(u32, u32)>, number: &u32) -> bool {
	for (min, max) in ranges {
		if number >= min && number <= max {
			return true;
		}
	}
	return false;
}

fn calculate(data: &Data) -> Result<Answer, &str> {
	let mut invalids = 0;
	let mut valids : Vec<Vec<u32>> = vec![];
	for nearby in &data.nearby {
		let mut ticket_is_valid = true;
		for num in nearby {
			let mut found = false;
			for ranges in &data.fields {
				if ranges_contains(&ranges, num) {
					found = true;
					break;
				}
			}
			if !found {
				ticket_is_valid = false;
				invalids += num;
			}
		}
		if ticket_is_valid {
			valids.push(nearby.clone());
		}
	}

	// Part B
	// This matrix represents a mapping of every column in the number
	// lists to numbered field in the fields array.
	let field_count = valids[0].len();
	let mut matrix : Vec<Vec<bool>> =
		(0..field_count).map(|_| (0..field_count).map(|_| true).collect()).collect();

	// We will loop through each value by column and remove from the matrix
	// the values that can't match up.  Then see where we are.
	for col in 0..field_count {
		for row in &valids {
			let number = row[col];
			for (field_index, field_ranges) in data.fields.iter().enumerate() {
				if !ranges_contains(&field_ranges, &number) {
					matrix[col][field_index] = false;
				}
			}
		}
	}

	// Now we need to use the process of elimination whereby we iteratively
	// find the numbers that can only be in one column and which therefore
	// preclude them being in other columns
	let mut ticket_index_to_field_index : HashMap<usize, usize> = HashMap::new();
	// We need to do this removal step up to field_count times
	for _ in 0..field_count {
		// Find the next number to remove
		let mut remove = 0;
		for (ticket_index, row) in matrix.iter().enumerate() {
			if ticket_index_to_field_index.get(&ticket_index) == None && row.iter().filter(|b| **b).count() == 1 {
				let mut field_index = 0;
				remove = loop {
					if row[field_index] { break field_index; }
					field_index += 1;
				};
				ticket_index_to_field_index.insert(ticket_index, remove);
			}
		}
		for i in 0..field_count {
			if ticket_index_to_field_index.get(&i) == None {
				matrix[i][remove] = false;
			}
		}
	}

	// Now for each feature we found, if its name starts with departure,
	// include our ticket's number at that same ticket index in a product.
	let destination_product = 
		ticket_index_to_field_index.iter()
			.filter(|(_, fid)| data.field_names[fid].starts_with("departure"))
			.map(|(tid, _fid)| data.mine[*tid] as u64)
			.product();
	
	Ok((invalids, destination_product))
}

fn load(filename: &str) -> Result<Data, String> {
	if let Ok(file) = fs::File::open(filename) {
		let mut fields : Vec<Vec<(u32, u32)>> = vec![];
		let mut field_names : BTreeMap<usize, String> = BTreeMap::new();
		let mut mine : Vec<u32> = vec![];
		let mut nearby : Vec<Vec<u32>> = vec![];

		let field_re = Regex::new("^(.*): ").unwrap();
		let ranges_re = Regex::new(r"(\d+)-(\d+)").unwrap();
		let numbers_re = Regex::new(r"(\d+)").unwrap();

		let mut ps = ParseState::Fields;
		for line in BufReader::new(file).lines() {
			let line = line.unwrap();
			match ps {
				ParseState::Fields => {
					if let Some(f) = field_re.captures(&line) {
						let field_name = f.get(1).unwrap().as_str();
						let range_matches = ranges_re.captures_iter(&line[f.get(1).unwrap().end()..]);
						let mut ranges : Vec<(u32, u32)> = vec![];
						for m in range_matches {
							ranges.push((
								m.get(1).unwrap().as_str().parse::<u32>().unwrap(),
								m.get(2).unwrap().as_str().parse::<u32>().unwrap(),
							))
						}
						field_names.insert(fields.len(), field_name.to_string());
						fields.push(ranges);
					} else if line == "your ticket:" {
						ps = ParseState::Mine;
					} else { continue; }
				},
				ParseState::Mine => {
					if line == "nearby tickets:" {
						ps =ParseState::Nearby;
					} else if line.is_empty() {
						continue;
					} else {
						for m in numbers_re.captures_iter(&line) {
							mine.push(m.get(1).unwrap().as_str().parse::<u32>().unwrap());
						}
					}
				},
				ParseState::Nearby => {
					let mut near_line : Vec<u32> = vec![];
					for m in numbers_re.captures_iter(&line) {
						near_line.push(m.get(1).unwrap().as_str().parse::<u32>().unwrap());
					}
					nearby.push(near_line);
				}
			}
		}

		 Ok(Data{
			fields,
			field_names,
			mine,
			nearby
		})
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

