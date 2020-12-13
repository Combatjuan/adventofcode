use std::env;
use std::fs;
use std::io::{BufRead, BufReader};
use std::process::exit;
use regex::Regex;

// SOME type
type Answer = i64;

#[derive(Copy, Clone, Debug)]
enum Direction {
	North,
	East,
	South,
	West,
}

#[derive(Debug)]
enum Action {
	Nop,
	North(i32),
	East(i32),
	South(i32),
	West(i32),
	Turn90,
	Turn180,
	Turn270,
	Forward(i32),
}

struct Ship {
	x: i32,
	y: i32,
	facing: Direction,
}

fn turn_90(facing: &Direction) -> Direction {
	match facing {
		Direction::North => Direction::West,
		Direction::East => Direction::North,
		Direction::South => Direction::East,
		Direction::West => Direction::South,
	}
}

fn turn_180(facing: &Direction) -> Direction {
	match facing {
		Direction::North => Direction::South,
		Direction::East => Direction::West,
		Direction::South => Direction::North,
		Direction::West => Direction::East,
	}
}

fn turn_270(facing: &Direction) -> Direction {
	match facing {
		Direction::North => Direction::East,
		Direction::East => Direction::South,
		Direction::South => Direction::West,
		Direction::West => Direction::North,
	}
}

impl Ship {
	fn new() -> Ship {
		Ship {
			x: 0,
			y: 0,
			facing: Direction::East
		}
	}

	fn go(&mut self, action: &Action) {
		let mut new_facing = self.facing.clone();
		let mut new_x = self.x;
		let mut new_y = self.y;

		match action {
			Action::North(how_far) => {
				new_y = self.y + how_far
			},
			Action::East(how_far) => {
				new_x = self.x + how_far
			},
			Action::South(how_far) => {
				new_y = self.y + -how_far
			},
			Action::West(how_far) => {
				new_x = self.x + -how_far
			},
			Action::Turn90 => {
				new_facing = turn_90(&self.facing)
			}
			Action::Turn180 => {
				new_facing = turn_180(&self.facing)
			}
			Action::Turn270 => {
				new_facing = turn_270(&self.facing)
			}
			Action::Forward(how_far) => {
				match self.facing {
					Direction::North => new_y += how_far,
					Direction::East => new_x += how_far,
					Direction::South => new_y -= how_far,
					Direction::West => new_x -= how_far,
				}
			},
			Action::Nop => {}
		}
		println!("Was: ({}, {}) facing {:?} did {:?} now at ({}, {}) facing {:?}",
			self.x, self.y, self.facing, action,
			new_x, new_y, new_facing);
		self.x = new_x;
		self.y = new_y;
		self.facing = new_facing;
	}

	fn distance(&self) -> Answer {
		(self.x.abs() as i32 + self.y.abs() as i32) as Answer
	}
}

fn calculate(data: &Vec<Action>) -> Result<Answer, &str> {
	let mut ship = Ship::new();
	for action in data {
		ship.go(action);
	}
	Ok(ship.distance())
}

fn parse_line(line: &String) -> Action {
	let re = Regex::new(r"([NESWLRF])(\d+)").unwrap();
	if let Some(cap) = re.captures(line) {
		let c = cap.get(1).unwrap().as_str().chars().next().unwrap();
		let n = cap.get(2).unwrap().as_str().parse::<i32>().unwrap();
		println!("Got '{}', c={}, n={}", line, c, n);
		match c {
			'N' => Action::North(n),
			'E' => Action::East(n),
			'S' => Action::South(n),
			'W' => Action::West(n),
			'F' => Action::Forward(n),
			'L' => {
				match n {
					90 => Action::Turn90,
					180 => Action::Turn180,
					270 => Action::Turn270,
					_ => Action::Nop,
				}
			},
			'R' => {
				match n {
					90 => Action::Turn270,
					180 => Action::Turn180,
					270 => Action::Turn90,
					_ => Action::Nop,
				}
			},
			_ => Action::Nop,
		}
	} else {
		println!("I got instruction '{}' but didn't understand it.", line);
		Action::Nop
	}
}

fn load(filename: &str) -> Result<Vec<Action>, String> {
	if let Ok(file) = fs::File::open(filename) {
		let lines = BufReader::new(file).lines();
		let data : Vec<Action> = lines.map(|x| parse_line(&x.unwrap())).collect();
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
		Ok(answer) => {
			println!("Answer: {}", answer);
			0
		},
		Err(err) => {
			println!("{}", err);
			1
		}
	});
}

