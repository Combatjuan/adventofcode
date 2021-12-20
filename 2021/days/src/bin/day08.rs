use std::env;
use std::fs;
use std::io::{BufRead, BufReader};
use std::process::exit;
use std::collections::{BTreeMap,BTreeSet};
use std::iter::FromIterator;
use regex::Regex;

// SOME type
type Answer = i64;

#[derive(Clone)]
struct Data {
	numbers: [BTreeSet<char>; 10],
	display: [BTreeSet<char>; 4],
}

fn chars_from_string(s: &String) -> BTreeSet<char> {
	BTreeSet::from_iter(s.chars())
}

fn parse_line(line_number: usize, line: &String) -> Result<Data, String> {
	let re = Regex::new(r"([a-g]+) ([a-g]+) ([a-g]+) ([a-g]+) ([a-g]+) ([a-g]+) ([a-g]+) ([a-g]+) ([a-g]+) ([a-g]+) \| ([a-g]+) ([a-g]+) ([a-g]+) ([a-g]+)").unwrap();
	if let Some(cap) = re.captures(line) {
		let mut numbers : [BTreeSet<char>; 10] = Default::default();
		let mut display : [BTreeSet<char>; 4] = Default::default();
		for i in 0..10 {
			numbers[i] = chars_from_string(&cap.get(i + 1).unwrap().as_str().to_string());
		}
		for i in 0..4 {
			display[i] = chars_from_string(&cap.get(i + 11).unwrap().as_str().to_string());
		}

		Ok(Data {
			numbers,
			display
		})
	} else {
		Err(format!("Failed to parse line {}: '{}'", line_number, line))
	} 
}

fn calculate_a(data: &Vec<Data>) -> Result<Answer, String> {
	let mut count : Answer = 0;
	for d in data {
		count += d.display.iter().filter(|x| [2, 3, 4, 7].contains(&x.len())).count() as Answer;
	}
	Ok(count)
}

fn display_to_number(display: &[BTreeSet<char>; 4], mapping: BTreeMap<BTreeSet<char>, u8>) -> Answer {
	*mapping.get(&display[0]).unwrap() as i64 * 1000
		+ *mapping.get(&display[1]).unwrap() as i64 * 100
		+ *mapping.get(&display[2]).unwrap() as i64 * 10
		+ *mapping.get(&display[3]).unwrap() as i64 * 1
}

fn calculate_b(data: &Vec<Data>) -> Result<Answer, String> {
	let mut sum : Answer = 0;

	for d in data {
		let mut mapping = BTreeMap::new();
		// Get the simple ones
		let one : BTreeSet<char> = d.numbers.iter().filter(|x| x.len() == 2).next().unwrap().clone();
		let four : BTreeSet<char> = d.numbers.iter().filter(|x| x.len() == 4).next().unwrap().clone();
		let seven : BTreeSet<char> = d.numbers.iter().filter(|x| x.len() == 3).next().unwrap().clone();
		let eight : BTreeSet<char> = d.numbers.iter().filter(|x| x.len() == 7).next().unwrap().clone();
		// Now the trickier cases
		let two : BTreeSet<char> = d.numbers.iter().filter(|x| x.len() == 5 && four.intersection(x).count() == 2).next().unwrap().clone();
		let three : BTreeSet<char> = d.numbers.iter().filter(|x| x.len() == 5 && one.intersection(x).count() == 2).next().unwrap().clone();
		let six : BTreeSet<char> = d.numbers.iter().filter(|x| x.len() == 6 && one.intersection(x).count() == 1).next().unwrap().clone();
		let five : BTreeSet<char> = d.numbers.iter().filter(|x| x.len() == 5 && six.intersection(x).count() == 5).next().unwrap().clone();
		let zero : BTreeSet<char> = d.numbers.iter().filter(|x| x.len() == 6 && five.intersection(x).count() == 4).next().unwrap().clone();
		let nine : BTreeSet<char> = d.numbers.iter().filter(|x| x.len() == 6 && four.intersection(x).count() == 4).next().unwrap().clone();

		mapping.insert(zero, 0);
		mapping.insert(one, 1);
		mapping.insert(two, 2);
		mapping.insert(three, 3);
		mapping.insert(four, 4);
		mapping.insert(five, 5);
		mapping.insert(six, 6);
		mapping.insert(seven, 7);
		mapping.insert(eight, 8);
		mapping.insert(nine, 9);

		sum += display_to_number(&d.display, mapping);
	}

	// Now we have to actually translate
	Ok(sum)
}

fn calculate(data: &Vec<Data>) -> (Result<Answer, String>, Result<Answer, String>) {
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
