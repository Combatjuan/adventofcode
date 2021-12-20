use std::cmp::max;
use std::env;
use std::fs;
use std::io::{BufRead, BufReader};
use std::process::exit;
use regex::Regex;

// SOME type
type Answer = i64;

#[derive(Clone)]
struct Line {
	x1: i64,
	y1: i64,
	x2: i64,
	y2: i64,
}

type Cell = u64;

struct Grid {
	cells: Vec<Cell>,
	width: i64,
	height: i64,
}

impl Grid {
	fn new(width: i64, height: i64) -> Grid {
		Grid {
			cells: vec![0; (width * height) as usize],
			width,
			height
		}
	}

	fn raise(self: &mut Grid, line: &Line) {
		let start_x = line.x1 as i64;
		let start_y = line.y1 as i64;
		let distance = max((line.x1 - line.x2).abs(), (line.y1 - line.y2).abs()) + 1;
		let dx : i64 = match line.x2 - line.x1 {
			x if x < 0 => -1,
			x if x > 0 => 1,
			_ => 0,
		};
		let dy : i64 = match line.y2 - line.y1 {
			y if y < 0 => -1,
			y if y > 0 => 1,
			_ => 0,
		};
		//println!("Line: ({}, {})->({}, {}) = ({}, {}) + Distance: {} * ({}, {})",
		//	line.x1, line.y1, line.x2, line.y2, start_x, start_y, distance, dx, dy);
		for n in 0..distance {
			let x = start_x + n * dx;
			let y = start_y + n * dy;
			self.cells[(y * self.width as i64 + x) as usize] += 1;
		}
	}

	fn count(self: &Grid) -> Answer {
		let mut answer = 0;
		for x in 0..self.width {
			for y in 0..self.height {
				if self.cells[(y * self.width + x) as usize] > 1 {
					answer += 1;
				}
			}
		}
		answer
	}

	fn print(self: &Grid) {
		for y in 0..self.height {
			for x in 0..self.width {
				let value = self.cells[(y * self.width + x) as usize];
				print!("{}",
					match value {
						0 => '.',
						1|2|3|4|5|6|7|8|9 => char::from_digit(value as u32, 10).unwrap(),
						_ => 'X',
					}
				)
			}
			println!();
		}
	}
}

fn bounds(data: &Vec<Line>) -> (i64, i64) {
	let mut max_x = 0;
	let mut max_y = 0;
	for d in data {
		if d.x1 > max_x {
			max_x = d.x1;
		}
		if d.x2 > max_x {
			max_x = d.x2;
		}
		if d.y1 > max_y {
			max_y = d.y1;
		}
		if d.y2 > max_y {
			max_y = d.y2;
		}
	}
	(max_x, max_y)
}

fn parse_line(line_number: usize, line: &String) -> Result<Line, String> {
	let re = Regex::new(r"(\d+),(\d+) -> (\d+),(\d+)").unwrap();
	if let Some(cap) = re.captures(line) {
		Ok(Line {
			x1: cap.get(1).unwrap().as_str().parse::<i64>().unwrap(),
			y1: cap.get(2).unwrap().as_str().parse::<i64>().unwrap(),
			x2: cap.get(3).unwrap().as_str().parse::<i64>().unwrap(),
			y2: cap.get(4).unwrap().as_str().parse::<i64>().unwrap(),
		})
	} else {
		Err(format!("Failed to parse line {}: '{}'", line_number, line))
	} 
}

fn calculate_a(data: &Vec<Line>) -> Result<Answer, String> {
	let (max_x, max_y) = bounds(data);
	let mut grid = Grid::new(max_x + 1, max_y + 1);
	for line in data {
		if line.x1 == line.x2 || line.y1 == line.y2 {
			grid.raise(line);
		}
	}
	Ok(grid.count())
}

fn calculate_b(data: &Vec<Line>) -> Result<Answer, String> {
	let (max_x, max_y) = bounds(data);
	let mut grid = Grid::new(max_x + 1, max_y + 1);
	for line in data {
		grid.raise(line);
	}
	grid.print();
	Ok(grid.count())
}

fn calculate(data: &Vec<Line>) -> (Result<Answer, String>, Result<Answer, String>) {
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

fn load(filename: &str) -> Result<Vec<Line>, String> {
	if let Ok(file) = fs::File::open(filename) {
		let lines = BufReader::new(file).lines();
		let data : Vec<Line> = lines.enumerate().map(
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
