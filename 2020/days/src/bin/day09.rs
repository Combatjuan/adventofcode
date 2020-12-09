use std::env;
use std::fs;
use std::io::{BufRead, BufReader};
use std::process::exit;

// SOME type
type Answer = (i64, i64);

const XMAS_LEN : usize = 25;

struct NumberTable {
    // The last <number_count> numbers
    history: Vec<i64>,
    // A vector of multiples of the last <number_count> numbers multiplied pairwise.
    valids: Vec<i64>,
    // Internally used to keep track of the next place to store data in our history and valid
    // circular buffers.
    ptr: usize,
    // Should be equivalent to valids.len()
    number_count: usize,
}

impl NumberTable {
    fn new(number_count: usize) -> NumberTable {
        let mut history = Vec::<i64>::new();
        history.resize(number_count, 0);
        let mut valids = Vec::<i64>::new();
        // We keep track of an array of all the valid numbers which is a vector sized at
        // n * (n-1).  For a large N, this is not an efficient implementation.  But for
        // a small n, iterating over a list of numbers and doing comparisons should be
        // very fast on a modern CPU relative to lookups.
        //
        // It's also true that this list will contain some duplicates.  It could be
        // (n * (n-1) / 2) in size but at the cost of increased complexity.
        valids.resize(number_count * number_count - 1, 0);
        NumberTable {
            history,
            valids,
            ptr: 0,
            number_count,
        }
    }

    // Takes a new number, puts it on its circuluar buffer of historical numbers
    // And updates its table of current valid numbers.
    fn add(&mut self, number: i64) {
        println!("Add {} at index {}", number, self.ptr);
        self.history[self.ptr] = number;

        // Update the valids that correspond in our circle-buffer to the current pointer.
        let mut vptr : usize = self.ptr * (self.number_count - 1);
        for (i, n) in self.history.iter().enumerate() {
            // Skip ourselves
            if i != self.ptr {
                self.valids[vptr] = number + n;
                println!("  Update index {} with {}", vptr, number + n);
                vptr += 1;
            }
        }

        // Update our internal pointer to accept the next value
        self.ptr += 1;
        if self.ptr == self.number_count {
            self.ptr = 0;
        }
    }

    fn is_valid(&self, number: i64) -> bool {
        self.valids.iter().any(|n| *n == number)
    }
}

// Given the data and answer from part a find the answer for part b
fn find_continuous(data: &Vec<i64>, weakness: i64) -> Option<i64> {
    for (i, num) in data.iter().enumerate() {
        let mut sum = 0;
        let mut j = i;
        while sum <= weakness && j < data.len() {
            sum += data[j];
            if sum == weakness {
                for k in i..(j+1) {
                    println!("{}", data[k]);
                }

                return Some(num + data[j]);
            }
            j += 1;
        }
    }
    None
}

fn calculate(data: &Vec<i64>) -> Result<Answer, &str> {
    let mut table = NumberTable::new(XMAS_LEN);
    // Populate the initial values:
    for i in 0..XMAS_LEN {
        table.add(data[i]);
    }
    // Now add numbers until one doesn't fit.
    for number in data.iter().skip(XMAS_LEN) {
        if table.is_valid(*number) {
            table.add(*number);
        } else {
            let result = find_continuous(&data, *number);
            if let Some(part_b) = result {
                return Ok((*number, part_b));
            } else {
                println!("Could not find continous range.");
                break;
            }
        }
    }
    Err("All the numbers were valid.")
}

fn parse_line(line: &String) -> i64 {
    line.parse::<i64>().unwrap()
}

fn load(filename: &str) -> Result<Vec<i64>, String> {
	if let Ok(file) = fs::File::open(filename) {
		let lines = BufReader::new(file).lines();
		let data : Vec<i64> = lines.map(|x| parse_line(&x.unwrap())).collect();
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
			println!("Invalid: {}", a);
			println!("Weakness: {}", b);
			0
		},
		Err(err) => {
			println!("{}", err);
			1
		}
	});
}

