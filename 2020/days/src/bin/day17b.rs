use std::env;
use std::fmt;
use std::fs;
use std::io::{BufRead, BufReader};
use std::process::exit;

// SOME type
type Answer = usize;

#[derive(Clone)]
struct Universe {
	atoms: Vec<Vec<Vec<Vec<bool>>>>,
	x_offset: i32,
	y_offset: i32,
	z_offset: i32,
		w_offset: i32,
	x_range: (i32, i32),
	y_range: (i32, i32),
	z_range: (i32, i32),
	w_range: (i32, i32),
}

const OFFSETS : [(i32, i32, i32, i32); 80] = [
	(-1, -1, -1, -1),
	(-1, -1, -1, 0),
	(-1, -1, -1, 1),
	(-1, -1, 0, -1),
	(-1, -1, 0, 0),
	(-1, -1, 0, 1),
	(-1, -1, 1, -1),
	(-1, -1, 1, 0),
	(-1, -1, 1, 1),
	(-1, 0, -1, -1),
	(-1, 0, -1, 0),
	(-1, 0, -1, 1),
	(-1, 0, 0, -1),
	(-1, 0, 0, 0),
	(-1, 0, 0, 1),
	(-1, 0, 1, -1),
	(-1, 0, 1, 0),
	(-1, 0, 1, 1),
	(-1, 1, -1, -1),
	(-1, 1, -1, 0),
	(-1, 1, -1, 1),
	(-1, 1, 0, -1),
	(-1, 1, 0, 0),
	(-1, 1, 0, 1),
	(-1, 1, 1, -1),
	(-1, 1, 1, 0),
	(-1, 1, 1, 1),
	(0, -1, -1, -1),
	(0, -1, -1, 0),
	(0, -1, -1, 1),
	(0, -1, 0, -1),
	(0, -1, 0, 0),
	(0, -1, 0, 1),
	(0, -1, 1, -1),
	(0, -1, 1, 0),
	(0, -1, 1, 1),
	(0, 0, -1, -1),
	(0, 0, -1, 0),
	(0, 0, -1, 1),
	(0, 0, 0, -1),
	(0, 0, 0, 1),
	(0, 0, 1, -1),
	(0, 0, 1, 0),
	(0, 0, 1, 1),
	(0, 1, -1, -1),
	(0, 1, -1, 0),
	(0, 1, -1, 1),
	(0, 1, 0, -1),
	(0, 1, 0, 0),
	(0, 1, 0, 1),
	(0, 1, 1, -1),
	(0, 1, 1, 0),
	(0, 1, 1, 1),
	(1, -1, -1, -1),
	(1, -1, -1, 0),
	(1, -1, -1, 1),
	(1, -1, 0, -1),
	(1, -1, 0, 0),
	(1, -1, 0, 1),
	(1, -1, 1, -1),
	(1, -1, 1, 0),
	(1, -1, 1, 1),
	(1, 0, -1, -1),
	(1, 0, -1, 0),
	(1, 0, -1, 1),
	(1, 0, 0, -1),
	(1, 0, 0, 0),
	(1, 0, 0, 1),
	(1, 0, 1, -1),
	(1, 0, 1, 0),
	(1, 0, 1, 1),
	(1, 1, -1, -1),
	(1, 1, -1, 0),
	(1, 1, -1, 1),
	(1, 1, 0, -1),
	(1, 1, 0, 0),
	(1, 1, 0, 1),
	(1, 1, 1, -1),
	(1, 1, 1, 0),
	(1, 1, 1, 1),
];

impl Universe {
	fn next(&self) -> Universe {
		let x_offset = self.x_range.0 - 1;
		let y_offset = self.y_range.0 - 1;
		let z_offset = self.z_range.0 - 1;
		let w_offset = self.w_range.0 - 1;
		let length = self.x_range.1 - self.x_range.0 + 3;
		let width = self.y_range.1 - self.y_range.0 + 3;
		let height = self.z_range.1 - self.z_range.0 + 3;
		let time = self.w_range.1 - self.w_range.0 + 3;
		let mut atoms : Vec<Vec<Vec<Vec<bool>>>> =
			(0..time).map(|_| (0..height).map(|_| 0..height).map(|_| (0..width).map(|_| (0..length).map(|_| false).collect()).collect()).collect()).collect();
		let mut x_range : (i32, i32) = (i32::MAX, i32::MIN);
		let mut y_range : (i32, i32) = (i32::MAX, i32::MIN);
		let mut z_range : (i32, i32) = (i32::MAX, i32::MIN);
		let mut w_range : (i32, i32) = (i32::MAX, i32::MIN);

		println!("Creating new universe size ({}, {}, {}, {}) from:
			xs: {}, ({}-{}),
			ys: {}, ({}-{}),
			zs: {}, ({}-{}),
			ws: {}, ({}-{})",
			length, width, height, time,
			self.x_offset, self.x_range.0, self.x_range.1,
			self.y_offset, self.y_range.0, self.y_range.1,
			self.z_offset, self.z_range.0, self.z_range.1,
			self.w_offset, self.w_range.0, self.w_range.1,
		);

		for w in (self.w_range.0 - 1)..(self.w_range.1 + 2) {
			for z in (self.z_range.0 - 1)..(self.z_range.1 + 2) {
				for y in (self.y_range.0 - 1)..(self.y_range.1 + 2) {
					for x in (self.x_range.0 - 1)..(self.x_range.1 + 2) {
						let active = self.at(x, y, z, w);
						let neighbors = self.adjacent(x, y, z, w);
						if !active && neighbors == 3
							|| active && (neighbors == 2 || neighbors == 3) {
							atoms[(w - w_offset) as usize]
								[(z - z_offset) as usize]
								[(y - y_offset) as usize]
								[(x - x_offset) as usize] = true;
							if x < x_range.0 { x_range.0 = x; }
							if x > x_range.1 { x_range.1 = x; }
							if y < y_range.0 { y_range.0 = y; }
							if y > y_range.1 { y_range.1 = y; }
							if z < z_range.0 { z_range.0 = z; }
							if z > z_range.1 { z_range.1 = z; }
							if w < w_range.0 { w_range.0 = w; }
							if w > w_range.1 { w_range.1 = w; }
						}					 
					}
				}
			}
		}
		println!("Generating new universe with:
			length: {},
			width: {},
			height: {},
			time: {},
			xs: {}, ({}-{}),
			ys: {}, ({}-{}),
			zs: {}, ({}-{}),
			ws: {}, ({}-{})",
			length, width, height, time,
			x_offset, x_range.0, x_range.1,
			y_offset, y_range.0, y_range.1,
			z_offset, z_range.0, z_range.1,
			w_offset, w_range.0, w_range.1,
		);

		Universe {
			atoms,
			x_offset,
			y_offset,
			z_offset,
			w_offset,
			x_range,
			y_range,
			z_range,
			w_range,
		}
	}

	fn adjacent(& self, x: i32, y: i32, z: i32, w: i32) -> i32 {
		OFFSETS.iter().map(|(dx, dy, dz, dw)| if self.at(x + dx, y + dy, z + dz, w + dw) {1} else {0}).sum()
	}

	fn at(&self, x: i32, y: i32, z: i32, w: i32) -> bool {
		if x < self.x_range.0 || x > self.x_range.1
			|| 	y < self.y_range.0 || y > self.y_range.1
			|| 	z < self.z_range.0 || z > self.z_range.1
			|| 	w < self.w_range.0 || w > self.w_range.1 {
				false
		} else {
			self.atoms
				[(w - self.w_offset) as usize]
				[(z - self.z_offset) as usize]
				[(y - self.y_offset) as usize]
				[(x - self.x_offset) as usize]
		}
	}

	fn count(&self) -> usize {
		let mut count : usize = 0;
		for space in &self.atoms {
			for slice in space {
				for row in slice {
					for cell in row {
						if *cell { count += 1 }
					}
				}
			}
		}
		count
	}
}

impl fmt::Display for Universe {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		for w in self.w_range.0..(self.w_range.1 + 1) {
			write!(f, "===============================================\n")?;
			write!(f, "Space {}\n", w)?;
			for z in self.z_range.0..(self.z_range.1 + 1) {
				write!(f, "Layer {}\n", z)?;
				for y in self.y_range.0..(self.y_range.1 + 1) {
					for x in self.x_range.0..(self.x_range.1 + 1) {
						if let Err(e) = write!(f, "{}", match self.at(
								x,
								y,
								z,
								w) {
							true => '#',
							false => '.',
						}) {
							return Err(e)
						}
					}
					write!(f, "\n")?;
				}
			}
		}
		Ok(())
	}
}

fn calculate(data: &Universe) -> Result<Answer, &str> {
	let mut u : Universe = (*data).clone();
	println!("{}", u);
	for _ in 0..6 {
		u = u.next();
		println!("{}", u);
	}
	Ok(u.count())
}

fn parse_line(line: &String) -> Vec<bool> {
	line.chars().map(|c|
		match c {
			'.' => false,
			'#' => true,
			_ => false,
		}
	).collect()
}

fn load(filename: &str) -> Result<Universe, String> {
	if let Ok(file) = fs::File::open(filename) {
		let lines = BufReader::new(file).lines();
		let slice : Vec<Vec<bool>> = lines.map(|x| parse_line(&x.unwrap())).collect();
		match slice.is_empty() {
			true => Err(format!("Failed to parse every line in {}", filename)),
			false => {
				let length = slice[0].len() as i32;
				let width = slice.len() as i32;
				Ok(Universe {
					atoms: vec![vec![slice]],
					x_offset: 0,
					y_offset: 0,
					z_offset: 0,
					w_offset: 0,
					x_range: (0, length - 1),
					y_range: (0, width - 1),
					z_range: (0, 0),
					w_range: (0, 0),
				})
			}
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

