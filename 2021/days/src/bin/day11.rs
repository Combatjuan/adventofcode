use std::env;
use std::fs;
use std::io::{BufRead, BufReader};
use std::process::exit;

// SOME type
type Answer = usize;

type Cell = i32;

struct Grid<T> {
	cells: Vec<T>,
	width: usize,
	height: usize,
}

fn none_or_greater(n: Cell, other: Option<&Cell>) -> bool {
	if let Some(o) = other {
		n < *o
	} else {
		true
	}
}

#[derive(Clone, Copy)]
struct GridIterator<'a, T> {
	grid: &'a Grid<T>,
	x: usize,
	y: usize,
}

impl<'a, T> Iterator for GridIterator<'a, T>
	where T: Clone + Copy + Default
{
	type Item = (usize, usize, Option<&'a T>);

	fn next(&mut self) -> Option<Self::Item> {
		self.x += 1;
		if self.x >= self.grid.width {
			self.x = 0;
			self.y += 1;
		}
		if self.y >= self.grid.height {
			None
		} else {
			Some((self.x, self.y, self.grid.get(self.x, self.y)))
		}
	}
}

impl<T> Grid<T>
	where T: Clone + Copy + Default
{
	fn new(width: usize, height: usize) -> Grid<T> {
		Grid {
			cells: vec![Default::default(); (width * height) as usize],
			width,
			height
		}
	}

	fn iter(self: &Grid<T>) -> GridIterator<T> {
		GridIterator {
			grid: self,
			x: 0,
			y: 0,
		}
	}

	fn clear(self: &mut Grid<T>, value: &T) {
		for i in self.cells.iter_mut() {
			*i = *value;
		}
	}

	fn from_vecs(numbers: &Vec<Vec<T>>) -> Grid<T> {
		if numbers.is_empty() {
			Grid::new(0, 0)
		} else {
			let width = numbers.first().unwrap().len();
			let height = numbers.len();
			let mut cells : Vec<T> = vec!{};
			for row in numbers {
				for n in row {
					cells.push(n.clone());
				}
			}
			Grid {
				cells,
				width,
				height
			}
		}
	}

	fn copy(self: &Grid<T>) -> Grid<T> {
		Grid {
			cells: self.cells.clone(),
			width: self.width,
			height: self.height,
		}
	}

	/*
	fn flood<F: Fn(&T) -> bool>(self: &Grid<T>, f: F, x: usize, y: usize) -> Vec<(usize, usize)> {
		let mut flood : Vec<(usize, usize)> = Default::default();
		let mut visited : HashSet<(usize, usize)> = Default::default();
		let mut to_visit : VecDeque<(usize, usize)> = VecDeque::new();
		to_visit.push_back((x, y));
		while let Some((x, y)) = to_visit.pop_front() {
			if !visited.contains(&(x, y)) {
				if let Some(cell) = self.get(x, y) {
					if f(cell) {
						flood.push((x, y));
						if x > 0 && !visited.contains(&(x - 1, y)) {
							to_visit.push_back((x - 1, y));
						}
						if y > 0 && !visited.contains(&(x, y - 1)) {
							to_visit.push_back((x, y - 1));
						}
						if x < self.width && !visited.contains(&(x + 1, y)) {
							to_visit.push_back((x + 1, y));
						}
						if y < self.height && !visited.contains(&(x, y + 1)) {
							to_visit.push_back((x, y + 1));
						}
					}
					visited.insert((x, y));
				}
			}
		}
		flood
	}
	*/

	fn get(self: &Grid<T>, x: usize, y: usize) -> Option<&T> {
		if x >= self.width || y >= self.height {
			None
		} else {
			Some(&self.cells[(y * self.width + x) as usize])
		}
	}

	fn set(self: &mut Grid<T>, x: usize, y: usize, value: &T) {
		if x < self.width && y < self.height {
			self.cells[(y * self.width + x) as usize] = *value;
		} else {
			panic!("Out of bounds ({}, {}) on grid with dimensions {}x{}",
				x, y, self.width, self.height);
		}
	}

	fn print<F: Fn(&T)->char>(self: &Grid<T>, f: F)
	{
		for y in 0..self.height {
			for x in 0..self.width {
				let value = &self.cells[(y * self.width + x) as usize];
				print!("{}", f(value));
			}
			println!();
		}
	}
}

fn increment_at(octs: &mut Grid<i32>, x: usize, y: usize) -> Option<&Cell>{
	if x < octs.width && y < octs.height {
		octs.cells[(y * octs.width + x) as usize] += 1;
		return octs.get(x, y);
	} else {
		panic!("Out of bounds ({}, {}) on grid with dimensions {}x{}",
			x, y, octs.width, octs.height);
	}
}

fn increment_energy(octs: &mut Grid<i32>) {
	for i in 0..(octs.width * octs.height) {
		octs.cells[i] += 1;
	}
}

fn parse_line(line: &String) -> Vec<i32> {
	let mut numbers : Vec<i32> = Vec::new();
	for c in line.chars() {
		numbers.push(
			match c {
				'0' => 0,
				'1' => 1,
				'2' => 2,
				'3' => 3,
				'4' => 4,
				'5' => 5,
				'6' => 6,
				'7' => 7,
				'8' => 8,
				'9' => 9,
				_   => -1
			}
		)
	}
	numbers
}

fn num_to_char(n: &Cell) -> char {
	match n {
		0 => '0',
		1 => '1',
		2 => '2',
		3 => '3',
		4 => '4',
		5 => '5',
		6 => '6',
		7 => '7',
		8 => '8',
		9 => '9',
		_ => '.',
	}
}

fn flash_at(mut octs: &mut Grid<Cell>, flashed_grid: &mut Grid<bool>, x: usize, y: usize) {
	let energy_level = *octs.get(x, y).unwrap();
	println!("  Flash: ({}, {}) -> {}", x, y, energy_level);
	flashed_grid.set(x, y, &true);

	for (dx, dy) in [
		(-1, -1), (0, -1), (1, -1),
		(-1, 0), (1, 0),
		(-1, 1), (0, 1), (1, 1),
	] {
		let nx : i64 = x as i64 + dx;
		let ny : i64 = y as i64 + dy;
		if nx >= 0 && nx < octs.width as i64 && ny >= 0 && ny < octs.height as i64 {
			if let Some(energy_level) = increment_at(&mut octs, nx as usize, ny as usize) {
				if energy_level > &9 && !(*flashed_grid.get(nx as usize, ny as usize).unwrap()) {
					flash_at(octs, flashed_grid, nx as usize, ny as usize);
				}
			}
			println!("		 \\ ({}, {})", nx, ny);
		}
	}
}

fn calculate_a(grid: &Grid<Cell>) -> Result<Answer, String> {
	let mut octs : Grid<Cell> = grid.copy();
	let mut flashes : Answer = 0;
	for _ in 0..100 {
		octs.print(num_to_char);

		// Initial increment
		increment_energy(&mut octs);
		println!("++++++++++");
		octs.print(num_to_char);
		println!();
		let mut flashed_grid : Grid<bool> = Grid::new(octs.width, octs.height);

		// Initial flashing
		for y in 0..octs.height {
			for x in 0..octs.width {
				let energy_level = *octs.get(x, y).unwrap();
				if energy_level > 9 && !(*flashed_grid.get(x, y).unwrap()) {
					flash_at(&mut octs, &mut flashed_grid, x, y);
				}
			}
		}

		// Followup flashing
		for y in 0..octs.height {
			for x in 0..octs.width {
				let energy_level = *octs.get(x, y).unwrap();
				if energy_level > 9 && !(*flashed_grid.get(x, y).unwrap()) {
					flash_at(&mut octs, &mut flashed_grid, x, y);
				}
			}
		}

		flashed_grid.print(|x| if *x { 'T' } else { 'F' });
		println!();

		// Reset
		for y in 0..flashed_grid.height {
			for x in 0..flashed_grid.width {
				if let Some(f) = flashed_grid.get(x, y) {
					if *f {
						octs.set(x, y, &0);
						flashes += 1;
					}
				}
			}
		}
	}
	Ok(flashes)
}

fn calculate_b(grid: &Grid<Cell>) -> Result<Answer, String> {
	let mut octs : Grid<Cell> = grid.copy();
	let mut step = 0;
	let mut flashes : Answer = 0;
	while flashes != octs.width * octs.height {
		flashes = 0;
		octs.print(num_to_char);

		// Initial increment
		increment_energy(&mut octs);
		println!("++++++++++");
		octs.print(num_to_char);
		println!();
		let mut flashed_grid : Grid<bool> = Grid::new(octs.width, octs.height);

		// Initial flashing
		for y in 0..octs.height {
			for x in 0..octs.width {
				let energy_level = *octs.get(x, y).unwrap();
				if energy_level > 9 && !(*flashed_grid.get(x, y).unwrap()) {
					flash_at(&mut octs, &mut flashed_grid, x, y);
				}
			}
		}

		// Followup flashing
		for y in 0..octs.height {
			for x in 0..octs.width {
				let energy_level = *octs.get(x, y).unwrap();
				if energy_level > 9 && !(*flashed_grid.get(x, y).unwrap()) {
					flash_at(&mut octs, &mut flashed_grid, x, y);
				}
			}
		}

		flashed_grid.print(|x| if *x { 'T' } else { 'F' });
		println!();

		// Reset
		for y in 0..flashed_grid.height {
			for x in 0..flashed_grid.width {
				if let Some(f) = flashed_grid.get(x, y) {
					if *f {
						octs.set(x, y, &0);
						flashes += 1;
					}
				}
			}
		}
		step += 1;
	}
	Ok(step)
}

fn calculate(grid: &Grid<Cell>) -> (Result<Answer, String>, Result<Answer, String>) {
	match calculate_a(grid) {
		Ok(a) => {
			match calculate_b(grid) {
				Ok(b) => (Ok(a), Ok(b)),
				Err(b) => (Ok(a), Err(b)),
			}
		},
		Err(a) => (Err(a), Err(format!("First solve A")))
	}
}

fn load(filename: &str) -> Result<Vec<Vec<i32>>, String> {
	if let Ok(file) = fs::File::open(filename) {
		let lines = BufReader::new(file).lines();
		let numbers : Vec<Vec<i32>> = lines.map(
				|x|
				parse_line(&x.unwrap())
			).collect();
		match numbers.is_empty() {
			true => Err(format!("Failed to parse every line in {}", filename)),
			false => Ok(numbers),
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
				match calculate(&Grid::<Cell>::from_vecs(&data)) {
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

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn flash_corner() {
		let cells = vec!{
			vec!{10, 0, 0},
			vec!{0, 0, 0},
			vec!{0, 0, 0},
		};
		let mut flashed = Grid::<bool>::new(3, 3);
		let mut octs = Grid::<Cell>::from_vecs(&cells);
		flash_at(&mut octs, &mut flashed, 0, 0);

		assert_eq!(*octs.get(0, 0).unwrap(), 10);
		assert_eq!(*octs.get(1, 0).unwrap(), 1);
		assert_eq!(*octs.get(2, 0).unwrap(), 0);
		assert_eq!(*octs.get(0, 1).unwrap(), 1);
		assert_eq!(*octs.get(1, 1).unwrap(), 1);
		assert_eq!(*octs.get(2, 1).unwrap(), 0);
		assert_eq!(*octs.get(0, 2).unwrap(), 0);
		assert_eq!(*octs.get(1, 2).unwrap(), 0);
		assert_eq!(*octs.get(2, 2).unwrap(), 0);
	}
	#[test]
	fn flash_center() {
		let cells = vec!{
			vec!{0, 0, 0},
			vec!{0, 10, 1},
			vec!{0, 0, 0},
		};
		let mut flashed = Grid::<bool>::new(3, 3);
		let mut octs = Grid::<Cell>::from_vecs(&cells);
		flash_at(&mut octs, &mut flashed, 1, 1);

		assert_eq!(*octs.get(0, 0).unwrap(), 1);
		assert_eq!(*octs.get(1, 0).unwrap(), 1);
		assert_eq!(*octs.get(2, 0).unwrap(), 1);
		assert_eq!(*octs.get(0, 1).unwrap(), 1);
		assert_eq!(*octs.get(1, 1).unwrap(), 10);
		assert_eq!(*octs.get(2, 1).unwrap(), 2);
		assert_eq!(*octs.get(0, 2).unwrap(), 1);
		assert_eq!(*octs.get(1, 2).unwrap(), 1);
		assert_eq!(*octs.get(2, 2).unwrap(), 1);
	}
}
