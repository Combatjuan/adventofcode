mod advent;

use regex::Regex;
use advent::{Calculator, StringVecParser, Solution, advent_exit};

struct Record {
	inventory: Vec<i64>,
}

struct CalorieCalculator {}
impl Calculator<Record, i64> for CalorieCalculator {
	fn solve_a(&mut self, records: &Vec<Record>) -> Result<i64, String> {
		let mut max : i64 = 0;
		for record in records {
			let mut inventory_total : i64 = 0;
			for c in &record.inventory {
				inventory_total += c;
			}
			if inventory_total > max {
				max = inventory_total;
			}
		}
		Ok(max)
	}
	fn solve_b(&mut self, records: &Vec<Record>) -> Result<i64, String> {
		let mut totals = vec![];
		for record in records {
			let mut inventory_total : i64 = 0;
			for c in &record.inventory {
				inventory_total += c;
			}
			totals.push(inventory_total);
		}

		totals.sort();
		if totals.len() < 3 {
			return Err("There were not at least 3 backpacks full.".to_string());
		} else {
			let l = totals.len();
			Ok(totals[l - 1] + totals[l -2] + totals[l - 3])
		}
	}
}

fn static_parser(lines: Vec<String>) -> Result<Vec<Record>, String> {
	let re = Regex::new(r"\d+").unwrap();
	let mut records = vec![];
	let mut inventory = vec![];
	for (line_number, line) in lines.iter().enumerate() {
		if line.is_empty() {
			if inventory.len() != 0 {
				records.push(Record{inventory: inventory.clone()});
			}
			inventory.clear();
		} else {
			if let Some(cap) = re.captures(&line) {
				let number : i64 = cap.get(0).unwrap().as_str().parse::<i64>().unwrap();
				inventory.push(number);
			} else {
				return Err(format!("Failed to parse line {}: '{}'", line_number, line));
			} 
		}
	}
	if inventory.len() != 0 {
		records.push(Record{inventory: inventory.clone()});
	}
	Ok(records)
}

fn main() {
	let mut parser = StringVecParser::new(&static_parser);
	let mut calculator = CalorieCalculator {};
	let status = Solution::solve::<Record, i64>(&mut parser, &mut calculator);
	advent_exit(status);
}
