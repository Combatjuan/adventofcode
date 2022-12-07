mod advent;

use std::process::exit;
use regex::Regex;
use advent::{Calculator, LineParser, Solution};

struct SumCalculator {}
impl Calculator<i64, i64> for SumCalculator {
	fn solve(&mut self, records: &Vec<i64>) -> Result<i64, String> {
		let mut n : i64 = 0;
		for r in records {
			n += r;
		}
		Ok(n)
	}
}

fn static_parse_line(line_number: usize, s: &String) -> Result<i64, String> {
	let re = Regex::new(r"\d+").unwrap();
	if let Some(cap) = re.captures(s) {
		let number : i64 = cap.get(0).unwrap().as_str().parse::<i64>().unwrap();
		Ok(number)
	} else {
		Err(format!("Failed to parse line {}: '{}'", line_number, s))
	} 
}

fn main() {
	let mut parser = LineParser::new(&static_parse_line);
	let mut calculator = SumCalculator {};
	exit(match Solution::solve::<i64, i64>(&mut parser, &mut calculator) {
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
