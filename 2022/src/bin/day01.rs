mod advent;

use std::process::exit;
use regex::Regex;
use advent::{Calculator, StringVecParser, Solution};

struct Record {
	inventory: Vec<i64>,
}

struct CalorieCalculator {}
impl Calculator<Record, i64> for CalorieCalculator {
	fn solve(&mut self, records: &Vec<Record>) -> Result<i64, String> {
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
	exit(match Solution::solve::<Record, i64>(&mut parser, &mut calculator) {
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

