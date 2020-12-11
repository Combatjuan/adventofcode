use std::env;
use std::fs;
use std::io::{BufRead, BufReader};
use std::process::exit;
use std::collections::BTreeMap;

// SOME type
type Answer = (i64, u64);

type Data = i64;

fn calculate(data: &Vec<Data>) -> Result<Answer, &str> {
	// Part A
	let mut adapters = data.clone();
	adapters.insert(0, 0);
	adapters.sort();
	// Add in the final device as an adapter for simplicity.
	adapters.push(adapters.last().unwrap() + 3);
	let mut ones = 0;
	let mut _twos = 0;
	let mut threes = 0;
	let mut jolts = 0;
	for j in adapters.iter().skip(1) {
		let diff = j - jolts;
		if diff == 1 {
			ones += 1;
		} else if diff == 2 {
			_twos += 1;
		} else if diff == 3 {
			threes += 1;
		} else {
			return Err("Invalid difference.");
		}
		jolts = *j;
	}

	// Part B
	// My intuition said that this was a combinations problem and that it therefore
	// involved lots of multiplication.  So my original solution was to create the
	// product of the number of ways to leave each node.
	//
	// That didn't work.  So I tried to take the product of the ways to get
	// to each node figuring that they would compound on one another.
	//
	// Alas.  At this point I drew some pictures and came to believe that
	// you can't multiply all the past possibilities or future possibilities
	// because sometimes nodes get skipped.
	//
	// But then I began to panic because it seemed like I may need a recursive
	// solution, but that would surely get to big as it became a combinitoric
	// explosion of previous paths.
	//
	// Then I think I came upon this simple truth:
	//
	//     ===================================================
	//	   The total number of ways to reach a node is the sum
	//	   of the ways to reach the nodes that can reach it.
	//     ===================================================
	//
	// So just addition?  In a combinations problem?  I am still surprised by
	// this but I think it is true because at each step the problem space is
	// relatively small.
	//
	// Therefore the solution is to create a map of what nodes can be reached
	// by what other nodes, initialize the 0th node to 1, and walk the graph 
	// summing as I go.  Let's give it at try!
	
	// Create a map of node index to list of nodes that point there
	let mut incoming_edges : BTreeMap<usize, Vec<usize>> = BTreeMap::new();
	for (i, _) in (&adapters).iter().enumerate() {
		// Check the next 3 indexes to see if they are destinations
		for j in std::cmp::min(i + 1, adapters.len())..(std::cmp::min(i + 4, adapters.len())) {
			if adapters[j] - adapters[i]  <= 3 {
				// Add the node we are iterating over in the outer loop
				// to the list of nodes that are incoming in our incoming_edges map.
				incoming_edges.entry(j).or_default().push(i);
			}
		}
	}
	
	// Now we will walk through our indexes and for each one look up how many ways there
	// were to get to each of the nodes that can reach this node.  That means we're both
	// building and referencing the counts array in this loop.  We do this by index
	// therefore making it a safe operation.
	let mut counts : Vec<u64> = vec![1];
	for (i, _) in (&adapters).iter().enumerate().skip(1) {
		let mut sum_of_possible_incoming : u64 = 0;
		let incoming = incoming_edges.get(&i).unwrap();
		for j in incoming {
			// Add an entry for this index to the counts array.  We'll reference it
			// on subsequent iterations from the nodes that this one can jump to.
			sum_of_possible_incoming += counts[*j];
		}
		counts.push(sum_of_possible_incoming);
	}
	
	Ok((ones * threes, *counts.last().unwrap()))
}

fn parse_line(line: &String) -> Data {
	line.parse::<i64>().unwrap()
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
			println!("Ones times Threes (a): {}", a);
			println!("Total Combinations (b): {}", b);
			0
		},
		Err(err) => {
			println!("{}", err);
			1
		}
	});
}

