mod advent;

use regex::Regex;
use advent::{advent_exit, Calculator, StringVecParser, Solution};

type Answer = String;

#[derive(Copy, Clone)]
struct Move {
	count: i64,
	from: usize,
	to: usize,
}

#[derive(Clone)]
#[derive(Debug)]
struct Pile {
	crates: Vec<char>,
}

#[derive(Clone)]
struct Pier {
	piles: Vec<Pile>,
}

impl Pier {
	fn move_crates_9000(&mut self, m: &Move) {
		let count = m.count;
		let from = m.from;
		let to = m.to;
		let from_index = from - 1;
		let to_index = to - 1;
		assert!(self.piles[from_index].crates.len() >= count as usize);

		for _ in 0..count {
			let c = self.piles[from_index].crates.pop().unwrap();
			self.piles[to_index].crates.push(c);
		}
	}

	fn move_crates_9001(&mut self, m: &Move) {
		let count = m.count;
		let from = m.from;
		let to = m.to;
		let from_index = from - 1;
		let to_index = to - 1;
		assert!(self.piles[from_index].crates.len() >= count as usize);

		let mut stack = vec![];
		for _ in 0..count {
			let c = self.piles[from_index].crates.pop().unwrap();
			stack.push(c);
		}

		for _ in 0..count {
			self.piles[to_index].crates.push(stack.pop().unwrap());
		}
	}
}

// Our input consists of two conceptuall different things;
//  1. A starting situation
//  2. A set of changes to that situation
//
//  We use the power of rust enums to capture both things in one type
//  (a record).  We then easily transform that collection of the two different
//  things into a single struct (a Puzzle).  Elegant.
//
//  Or maybe not elegant.  Maybe just a not-terrible way to keep me from having
//  to make my Parser/Calculator framework more general...
enum Record {
	Move(Move),
	Pier(Pier),
}

// 
struct Puzzle {
	pier: Pier,
	moves: Vec<Move>,
}

impl Puzzle {
	fn new(records: &Vec<Record>) -> Puzzle {
		let mut pier : Option<Pier> = None;
		let mut moves : Vec<Move> = vec![];
		for r in records {
			match r {
				Record::Move(m) => { moves.push(*m); }
				Record::Pier(p) => { pier = Some(p.clone()); }
			}
		}

		assert!(pier.is_some());

		Puzzle {
			pier: pier.unwrap(),
			moves
		}
	}
}

fn crane_parser(lines: Vec<String>) -> Result<Vec<Record>, String> {
	//let pile_re = Regex::new(r"^((   |\[[A-Z]\]) ?)+").unwrap();
	let is_pile_re = Regex::new(r"\[").unwrap();
	//let labels_re = Regex::new(r"^( [0-9] )+").unwrap();
	let is_label_re = Regex::new(r"^(\s*\d+\s*)+").unwrap();
	let labels_re = Regex::new(r"[0-9] ").unwrap();
	let move_re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
	let mut records = vec![];
	let mut piles : Vec<Pile> = vec![];

	for line in lines {
		if is_pile_re.is_match(&line) {
			// Each crate is 3 characters and then as space
			// Except the one on the end.
			assert!((line.len() + 1) % 4 == 0);
			let pile_count = (line.len() + 1) / 4;

			// Initialize piles if it hasn't happened yet
			if piles.len() == 0 {
				piles.resize(pile_count, Pile{crates: vec![]});
			}

			// Verify that we found the same number of crates (or places where
			// there could be crates but aren't) as we previously have.
			if piles.len() != pile_count {
				panic!("Inconsistent pile counts: {}", line);
			} 

			for pile_index in 0..piles.len() {
				let char_index = pile_index * 4 + 1;
				let c = line.chars().nth(char_index).unwrap();
				if c >= 'A' && c <= 'Z' {
					piles[pile_index].crates.push(c);
				} else {
				}
			}
		} else if is_label_re.is_match(&line) {
			let label_count = labels_re.find_iter(&line).count();
			let mut reversed_piles = vec![];
			assert!(label_count == piles.len());

			// We need to reverse each pile before we throw it on the pier
			// We defined it from the top down but it needs to be from the 
			// bottom up.  It's a *stack* of crates you see.
			for pile in piles.iter() {
				let mut reversed = pile.clone();
				reversed.crates.reverse();
				reversed_piles.push(reversed)
			}

			records.push(
				Record::Pier(
					//Pier { piles: piles.clone().reverse() }
					Pier { piles: reversed_piles }
				)
			);
		} else if let Some(cap) = move_re.captures(&line) {
			let count = cap.get(1).unwrap().as_str().parse::<i64>().unwrap() as i64;
			let from = cap.get(2).unwrap().as_str().parse::<i64>().unwrap() as usize;
			let to = cap.get(3).unwrap().as_str().parse::<i64>().unwrap() as usize;
			records.push(
				Record::Move(Move {
					count,
					from,
					to
				})
			);
		}
	}

	Ok(records)
}

struct CraneCalculator {}
impl Calculator<Record, Answer> for CraneCalculator {
	fn solve_a(&mut self, records: &Vec<Record>) -> Result<Answer, String> {
		let puzzle = Puzzle::new(&records);
		let mut pier = puzzle.pier.clone();
		for m in puzzle.moves {
			pier.move_crates_9000(&m);
		}

		let mut answer: String = "".to_string();
		for pile in pier.piles {
			answer.push(*pile.crates.last().unwrap());
		}
		Ok(answer)
	}
	fn solve_b(&mut self, records: &Vec<Record>) -> Result<Answer, String> {
		let puzzle = Puzzle::new(&records);
		let mut pier = puzzle.pier.clone();
		for m in puzzle.moves {
			pier.move_crates_9001(&m);
		}

		let mut answer: String = "".to_string();
		for pile in pier.piles {
			answer.push(*pile.crates.last().unwrap());
		}
		Ok(answer)
	}
}

fn main() {
	let mut parser = StringVecParser::new(&crane_parser);
	let mut calculator = CraneCalculator {};
	let status = Solution::solve::<Record, Answer>(&mut parser, &mut calculator);
	advent_exit(status);
}

