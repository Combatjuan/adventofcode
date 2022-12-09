mod advent;

use regex::Regex;
use advent::{advent_exit, Calculator, LineParser, Solution};

type Answer = i64;
type Record = i64;

fn static_parse_line(line_number: usize, s: &String) -> Result<Record, String> {
	let re = Regex::new(r"\d+").unwrap();
	if let Some(cap) = re.captures(s) {
		let number : i64 = cap.get(0).unwrap().as_str().parse::<i64>().unwrap();
		Ok(number)
	} else {
		Err(format!("Failed to parse line {}: '{}'", line_number, s))
	} 
}

struct SumCalculator {}
impl Calculator<Record, Answer> for SumCalculator {
	fn solve_a(&mut self, records: &Vec<Record>) -> Result<Answer, String> {
		let mut n : i64 = 0;
		for r in records {
			n += r;
		}
		Ok(n)
	}
	fn solve_b(&mut self, _records: &Vec<Record>) -> Result<Answer, String> {
		Err("Unimplemented".to_string())
	}
}

fn main() {
	let mut parser = LineParser::new(&static_parse_line);
	let mut calculator = SumCalculator {};
	let status = Solution::solve::<Record, Answer>(&mut parser, &mut calculator);
	advent_exit(status);
}
