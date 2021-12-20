use std::env;
use std::fs;
use std::io::{BufRead, BufReader};
use std::process::exit;

// SOME type
type Answer = u64;

#[derive(Clone)]
struct Data {
	fish: Vec<u8>
}

type FishBucket = Vec<u64>;

fn fish_to_buckets(fish: &Vec<u8>) -> FishBucket {
	let mut buckets : FishBucket = vec!{0, 0, 0, 0, 0, 0, 0, 0, 0};
	for i in 0..(9 as u8) {
		buckets[i as usize] = fish.iter().filter(|&f| *f == i).count() as u64;
	}
	buckets
}

fn str_to_numbers(s: &String) -> Vec<u8> {
	s.split(",").map(|x| x.parse::<u8>().unwrap()).collect()
}

fn parse_line(line: &String) -> Result<Data, String> {
	Ok(Data {
		fish: str_to_numbers(line)
	})
}

fn make_fish(data: &Data, days: i32) -> Result<Answer, String> {
	let mut fish = data.fish.clone();
	for day in 0..days {
		println!("Day: {}, Fish: {}", day, fish.len());
		let mut new_fish : Vec<u8> = vec!();
		for f in fish.iter_mut() {
			if *f == 0 {
				*f = 6;
				new_fish.push(8);
			} else {
				*f -= 1;
			}
		}
		fish.extend(new_fish);
	}
	Ok(fish.len() as u64)
}

fn calculate_fish(data: &Data, days: i32) -> Result<Answer, String> {
	let mut buckets = fish_to_buckets(&data.fish);
	for day in 0..days {
		println!("Day: {}, Fish: {}", day, buckets.iter().sum::<u64>());
		println!("Buckets: {:?}", buckets);
		let mut next_buckets : FishBucket = vec!{0, 0, 0, 0, 0, 0, 0, 0, 0};
		for (i, &count) in buckets.iter().enumerate() {
			if i == 0 {
				next_buckets[0] = 0;
				next_buckets[6] = count;
				next_buckets[8] = count;
			} else {
				next_buckets[i - 1] += count;
			}
		}
		buckets = next_buckets;
	}
	Ok(buckets.iter().sum::<u64>())
}

fn calculate_a(data: &Data) -> Result<Answer, String> {
	const DAYS : i32 = 80;
	make_fish(data, DAYS)
}

fn calculate_b(data: &Data) -> Result<Answer, String> {
	const DAYS : i32 = 256;
	calculate_fish(data, DAYS)
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
