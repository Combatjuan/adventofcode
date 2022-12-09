mod advent;

use regex::Regex;
use advent::{advent_exit, Calculator, LineParser, Solution};

struct Range{
    start: i64,
    end: i64,
}

impl Range{
    fn new(start: i64, end: i64) -> Range {
        Range { start, end }
    }
}

struct RangePair {
    range1: Range,
    range2: Range,
}

// 2-4,3-6
// 1-5,9-10
// 4-5,3-4

impl RangePair {
    fn contains_overlap(&self) -> bool {
        if (self.range1.start >= self.range2.start && self.range1.start <= self.range2.end)
            || (self.range2.start >= self.range1.start && self.range2.start <= self.range1.end)
            || (self.range1.end >= self.range2.start && self.range1.end <= self.range2.end)
            || (self.range2.end >= self.range1.start && self.range2.end <= self.range1.end)
        {
            true
        } else {
            false
        }
    }

    fn contains_superset(&self) -> bool {
        if (self.range1.start <= self.range2.start && self.range1.end >= self.range2.end)
            || (self.range2.start <= self.range1.start && self.range2.end >= self.range1.end)
        {
            true
        } else {
            false
        }
    }
}

type Answer = i64;
type Record = RangePair;

struct RangeCalculator {}
impl Calculator<Record, Answer> for RangeCalculator {
	fn solve_a(&mut self, records: &Vec<Record>) -> Result<Answer, String> {
        Ok(records.iter().filter(|r| r.contains_superset()).count() as i64)
	}
	fn solve_b(&mut self, records: &Vec<Record>) -> Result<Answer, String> {
        Ok(records.iter().filter(|r| r.contains_overlap()).count() as i64)
	}
}

fn static_parse_line(line_number: usize, s: &String) -> Result<Record, String> {
	let re = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();
	if let Some(cap) = re.captures(s) {
		let r1_start : i64 = cap.get(1).unwrap().as_str().parse::<i64>().unwrap();
		let r1_end : i64 = cap.get(2).unwrap().as_str().parse::<i64>().unwrap();
		let r2_start : i64 = cap.get(3).unwrap().as_str().parse::<i64>().unwrap();
		let r2_end : i64 = cap.get(4).unwrap().as_str().parse::<i64>().unwrap();
		Ok(RangePair {
            range1: Range::new(r1_start, r1_end),
            range2: Range::new(r2_start, r2_end),
        })
	} else {
		Err(format!("Failed to parse line {}: '{}'", line_number, s))
	} 
}

fn main() {
	let mut parser = LineParser::new(&static_parse_line);
	let mut calculator = RangeCalculator {};
	let status = Solution::solve::<Record, Answer>(&mut parser, &mut calculator);
	advent_exit(status);
}
