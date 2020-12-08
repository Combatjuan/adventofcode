use std::env;
use std::fs;
use std::io::{BufRead, BufReader};
use std::process::exit;
use std::collections::{BTreeMap, HashSet};
use regex::Regex;

type Answer = (i32, i32);

struct Data {
	outer: String,
	inner: Vec<(i32, String)>,
}

fn find_containers(color: &String, contained_in_map: &BTreeMap<String, Vec<String>>, mut checked: &mut HashSet<String>) {
	// If we already checked this color, short circuit
	if checked.get(color) == None {
		println!("-> {}",  color);
		// Otherwise add it to the list of colors we have checked
		checked.insert(color.clone());
		// Now we check and see if this color is contained in any other colors
		if let Some(_) = contained_in_map.get(color) {
			// This color is contained in some other colors.  Let's recursively search those.
			for to_check in contained_in_map.get(color).unwrap() {
				find_containers(to_check, contained_in_map, &mut checked);
			}
		} else {
			// The color is not contained in anything else.  Neat.
		}
	}
}

fn count_contains(color: &String, contains: &BTreeMap<String, &Data>) -> i32 {
	if let Some(datum) = contains.get(color) {
		assert!(*color == datum.outer);
		let mut count = 1;
		println!("Inside {} is:", color);
		for (inside_number, inside_color) in datum.inner.iter() {
			println!("  {} {}", inside_number, inside_color);
			count += inside_number * count_contains(&inside_color, &contains);
		}
		println!("Done: {}", count);
		count
	} else {
		1
	}
}

fn calculate(data: &Vec<Data>) -> Result<Answer, &str> {
	// ------
	// Part A
	// We need to invert the relationship to see what is contained in what else.
	// Build such a reverse mapping.
	let mut contained_in_map : BTreeMap<String, Vec<String>> = BTreeMap::new();
	for datum in data {
		for (_number, name) in datum.inner.iter() {
			if let Some(contained_by) = contained_in_map.get_mut(name) {
				contained_by.push(datum.outer.clone());
			} else {
				contained_in_map.insert(name.clone(), vec![datum.outer.clone()]);
			}
		}
	}

	// Now recursively search the reverse map storing each branch we visit in checked.
	let mut checked : HashSet<String> = HashSet::new();
	find_containers(&"shiny gold".to_string(), &contained_in_map, &mut checked);
	// Each node we checked is a color that contains "shiny gold" except that color
	// itself, so we subtract one.
	let contained_in_count = checked.len() as i32 - 1;

	// ------
	// Part B

	// Construct a map of what contains what
	let mut contains_map : BTreeMap<String, &Data> = BTreeMap::new();
	for datum in data {
		contains_map.insert(datum.outer.clone(), &datum);
	}
	let contains_count = count_contains(&"shiny gold".to_string(), &contains_map) - 1;

	Ok((contained_in_count, contains_count))
}

fn parse_line(line: &String) -> Data {
	let header_regex = Regex::new(
		r"([[:alpha:]]+ [[:alpha:]]+) bags contain").unwrap();
	if let Some(header) = header_regex.captures(line.as_str()) {
		let header_match = header.get(1).unwrap();
		let outer : String = header_match.as_str().to_string();
		let inner_regex = Regex::new(r" (\d+) ([[:alpha:]]+ [[:alpha:]]+) bags?[.,]").unwrap();
		let mut captures = inner_regex.captures_iter(&line[header_match.end()..]);
		let mut inner : Vec<(i32, String)> = vec![];
		while let Some(inner_match) = captures.next() {
			let number : i32 = inner_match.get(1).unwrap().as_str().parse::<i32>().unwrap();
			let bag : String = inner_match.get(2).unwrap().as_str().to_string();
			inner.push((number, bag));
		}
		Data {
				outer,
				inner,
		}
	} else {
		Data {
			outer: "None".to_string(),
			inner: vec![],
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

