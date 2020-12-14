use std::env;
use std::fs;
use std::io::{BufRead, BufReader};
use std::process::exit;
use regex::Regex;

// SOME type
type Answer = (u64, u64);

struct Data {
	earliest: u64,
	buses: Vec<Option<u64>>,
}

fn calculate(data: &Data) -> Result<Answer, &str> {
	// Part A
	// Zip the number of minutes until each bus leaves next with its bus id.
	let all_data = data.buses.clone();
	let answer_a =
	{
		let buses : Vec<u64> = all_data.into_iter().filter_map(|x| x).map(|x| x.clone()).collect();
		let mut bus_leaves_in : Vec<u64> = buses
			// Earliest mod bus_id yields the number of minutes ago the bus last arrived.
			// We subtract that from its bus id to know how many minutes until it will leave.
			// And we modulo it with itself to ensure that x gets turned into 0 where appropriate.
			.iter()
			.map(|x| (x - data.earliest % x) % x)
			.collect();
		// Sorting gives us the lowest number of minutes to leave in the first item.
		bus_leaves_in.sort();
		let schedule : Vec<(&u64, u64)> = buses.iter().zip(bus_leaves_in).collect();
		let bus = schedule.first().unwrap();
		// Answer is the multiple of the parts of the tuple
		bus.0 * bus.1
	};

	// Part B
	// Naive implementation which works for the example input
	// but is too slow for the real input.
	//
	// Yup: Turns out hte final answer was 1,058,443,396,696,792
	// which is a lot of calculations even for a modern processor.
	// Still, I might have gotten it done running all night on a fast CPU
	// with lots of cores.  The problem should be trivially parallellizable.
	//
	//let answer_b = {
	//	let buses = data.buses.clone();
	//	// Part B
	//	let mut nums : Vec<(usize, u64)> = vec![];
	//	for (i, option_n) in buses.iter().enumerate() {
	//		if let Some(n) = option_n {
	//			nums.push((i, *n));
	//		}
	//	}
	//	println!("Nums list: {:?}", nums);
	//	let mut n = 0;
	//	loop {
	//		if nums.iter().all(|(i, x)| (x - n % x) % x == (*i as u64)) {
	//			break n;
	//		}
	//		n += 1;
	//	}
	//};

	// After doing a simpler example by hand, I have a better way to do it.
	// 
	// The strategy is to keep incrementing until we find a number with all the
	// right offsets.  The optimization is that once we find numbers that are in lockstep
	// we can be sure that they won't be at the correct relative offsets again until
	// the current position plus multiples of their product (again, assuming they are
	// co-prime), which I'm not bothering to check but could easily do with the
	// factor library or rolling my own quickly.
	//
	// So we can start by increasing by some number, ideally the biggest number at
	// the correct offset and when we find another number that matches up to its offset,
	// we multiply our current incrementer by that number and remove it from the list of
	// numbers we care about.  We do this until all the numbers have been used.
	//
	// This is much faster than the previous implementation because rather than incrementing
	// by 1, we increment by an exponentially increasing number.  Reaching our final 16-digit
	// solution requires fewer than 700 iterations.
	let answer_b = {
		let mut nums = Vec::<(u64, u64)>::new();
		let buses = data.buses.clone();
		// Generate the offset at which we expect each number
		for (i, option_n) in buses.iter().enumerate().rev() {
			if let Some(n) = option_n {
				// The offset can exceed n which causes overlow on subtraction in Rust
				// So we need to make sure we modulo the offset with n first.
				// We also need to modulo the answer again to avoid the case where
				// it is the zeroeth position.
				nums.push(((n - (i as u64 % n)) % n, *n));
			}
		}

		let (mut offset, mut number) = nums.pop().unwrap();
		// Current is our counter.
		let mut current = 0;
		// Increment is the amount we'll increment each loop.
		// It will be the product of the numbers we're on the correct offset
		// of already.
		let mut incrementor = 1;
		loop {
			println!("{}	+{}, Number: {}, Offset: {}",
					 current, incrementor, number, offset);
			// If the number we are on modulo the number we're currently searching
			// for is at the correct offset, then we've found another number.
			if current % number == offset {
				// We multiply our incrementor by this number since we can be certain
				// that all future matches including it will be at that multiple.
				incrementor *= number;

				// And if we still have numbers left, we grab the next offset and
				// number to look for.
				if let Some(next) = nums.pop() {
					offset = next.0;
					number = next.1;
				// Otherwise we have found all the numbers and therefore have our answer.
				} else {
					break current;
				}
			}
			current += incrementor;
		}
	};

	Ok((answer_a, answer_b))
}

fn parse_line(line: &String) -> Vec<Option<u64>> {
	let mut buses : Vec<Option<u64>> = vec![];
	let re = Regex::new(r"(\d+|x)").unwrap();
	for m in  re.find_iter(line) {
		if m.as_str() == "x" {
			buses.push(None)
		} else {
			buses.push(Some(m.as_str().parse::<u64>().unwrap()));
		}
	}
	buses
}

fn load(filename: &str) -> Result<Data, String> {
	if let Ok(file) = fs::File::open(filename) {
		let mut lines = BufReader::new(file).lines();
		let earliest = lines.next().unwrap().unwrap().parse::<u64>().unwrap();
		let buses = parse_line(&lines.next().unwrap().unwrap());
		match buses.is_empty() {
			true => Err(format!("Failed to parse every line in {}", filename)),
			false => Ok(Data {
				earliest: earliest,
				buses: buses,
			}),
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
			println!("Answer a: {}", a);
			println!("Answer b: {}", b);
			0
		},
		Err(err) => {
			println!("{}", err);
			1
		}
	});
}

