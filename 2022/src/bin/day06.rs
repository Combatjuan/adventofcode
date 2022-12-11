mod advent;

use std::collections::HashSet;
use std::iter::FromIterator;
use std::fs::{File};
use std::io::{BufRead, BufReader};
use advent::{advent_exit, Calculator, Parser, Solution};

type Answer = i64;
type Record = char;

struct SingleLineParser {}
impl Parser<Record> for SingleLineParser {
    fn parse(&mut self, file: File) -> Result<Vec<Record>, String> {
        let mut line = String::new();
		let mut reader = BufReader::new(file);
        if let Ok(_) = reader.read_line(&mut line) {
            let mut records : Vec<Record> = vec![];
            for x in line.chars() {
                records.push(x as char);
            }
            Ok(records)
        } else {
            Err("Failed to read line from file.".to_string())
        }
    }
}

fn four_the_same(signals: &Vec<Record>, i: usize) -> bool {
    if i > signals.len() - 4 {
        false
    } else {
        let a = signals[i];
        let b = signals[i + 1];
        let c = signals[i + 2];
        let d = signals[i + 3];
        a != b && a != c && a != d && b != c && b != d && c != d
    }
}

fn all_unique(signals: &Vec<Record>, i: usize, n: usize) -> bool {
    if i > signals.len() - n {
        false
    } else {
        let sig_sub : Vec<char> = signals.iter().skip(i).map(|c| *c).take(n).collect();
        let set : HashSet<char> = HashSet::from_iter(sig_sub.iter().cloned());
        set.len() == n
    }
}

struct SumCalculator {}
impl Calculator<Record, Answer> for SumCalculator {
	fn solve_a(&mut self, records: &Vec<Record>) -> Result<Answer, String> {
        for i in 0..records.len() {
            if four_the_same(&records, i) {
                return Ok(i as i64 + 4)
            }
        }
        Err("Could not find start of signal marker".to_string())
	}
	fn solve_b(&mut self, records: &Vec<Record>) -> Result<Answer, String> {
        const UNIQUE_COUNT : usize = 14;
        for i in 0..records.len() {
            if all_unique(&records, i, UNIQUE_COUNT) {
                return Ok(i as i64 + UNIQUE_COUNT as i64)
            }
        }
        Err("Could not find start of signal marker".to_string())
	}
}

fn main() {
	let mut parser = SingleLineParser{};
	let mut calculator = SumCalculator {};
	let status = Solution::solve::<Record, Answer>(&mut parser, &mut calculator);
	advent_exit(status);
}
