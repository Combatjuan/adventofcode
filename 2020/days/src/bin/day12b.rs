use std::env;
use std::fs;
use std::io::{BufRead, BufReader};
use std::process::exit;
use regex::Regex;

// SOME type
type Answer = i64;

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
	wx: i32,
	wy: i32
}

impl Ship {
	fn new() -> Ship {
		Ship {
			x: 0,
			y: 0,
			wx: 10,
			wy: 1,
		}
	}

	fn go(&mut self, action: &Action) {
		let mut new_x = self.x;
		let mut new_y = self.y;
		let mut new_wx = self.wx;
		let mut new_wy = self.wy;

		match action {
			Action::North(how_far) => {
				new_wy += how_far
			},
			Action::East(how_far) => {
				new_wx += how_far
			},
			Action::South(how_far) => {
				new_wy -= how_far
			},
			Action::West(how_far) => {
				new_wx -= how_far
			},
			Action::Turn90 => {
				new_wx = -self.wy;
				new_wy = self.wx;
			}
			Action::Turn180 => {
				new_wx = -self.wx;
				new_wy = -self.wy;
			}
			Action::Turn270 => {
				new_wx = self.wy;
				new_wy = -self.wx;
			}
			Action::Forward(how_many) => {
				for _ in 0..*how_many {
					new_x += self.wx;
					new_y += self.wy;
				}
			},
			Action::Nop => {}
		}
		println!("Was: ({}, {}) waypoint ({}, {})  did {:?} now at ({}, {}) waypoint ({}, {})",
			self.x, self.y, self.wx, self.wy,
			action,
			new_x, new_y, new_wx, new_wy);
		self.x = new_x;
		self.y = new_y;
		self.wx = new_wx;
		self.wy = new_wy;
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

