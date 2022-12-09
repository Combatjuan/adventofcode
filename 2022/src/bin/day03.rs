mod advent;

use ascii::{ToAsciiChar, AsciiChar};
use advent::{advent_exit, Calculator, LineParser, Solution};
use std::collections::HashSet;

struct Rucksack {
    small_items: HashSet<u8>,
    large_items: HashSet<u8>,
}

type Record = Rucksack;
type Answer = i64;

fn rucksack_parser(line_number: usize, s: &String) -> Result<Record, String> {
    let item_count = s.len();
    if item_count % 2 == 1 {
        return Err(format!("Invalid number of items on line {}: '{}'", line_number, s));
    }

    let mut small_items = HashSet::<u8>::new();
    for c in s.chars().take(item_count / 2) {
        if c < 'A' || (c > 'Z' && c < 'a') || c > 'z' {
            return Err(format!("Invalid character '{}' found on line {} in string '{}'", c, line_number, s));
        } else {
            small_items.insert(c.to_ascii_char().unwrap().as_byte());
        }
    }

    let mut large_items = HashSet::<u8>::new();
    for c in s.chars().skip(item_count / 2).take(item_count / 2) {
        if c < 'A' || (c > 'Z' && c < 'a') || c > 'z' {
            return Err(format!("Invalid character '{}' found on line {} in string '{}'", c, line_number, s));
        } else {
            large_items.insert(c.to_ascii_char().unwrap().as_byte());
        }
    }
    Ok(
        Rucksack {
            small_items,
            large_items,
        }
    )
}

fn score_item(c: u8) -> i64 {
    if c >= AsciiChar::a && c <= AsciiChar::z {
        (c - AsciiChar::a.as_byte() + 1) as i64
    } else if c >= AsciiChar::A.as_byte() && c <= AsciiChar::Z.as_byte() {
        (c - AsciiChar::A.as_byte() + 27) as i64
    } else {
        0
    }
}

struct RucksackCalculator {}
impl Calculator<Record, Answer> for RucksackCalculator {
	fn solve_a(&mut self, records: &Vec<Record>) -> Result<Answer, String> {
        let mut score = 0;
        for record in records {
            let common = record.small_items.intersection(&record.large_items).collect::<Vec<&u8>>();
            assert!(common.len() == 1);
            let c = **common.iter().nth(0).unwrap();
            score += score_item(c);
        }
        Ok(score)
	}
	fn solve_b(&mut self, records: &Vec<Record>) -> Result<Answer, String> {
        assert!(records.len() % 3 == 0);
        let mut score = 0;
        let mut iter = records.iter();
        while let Some(first) = iter.next() {
            let second = iter.next().unwrap();
            let third = iter.next().unwrap();

            let union1 = &first.small_items | &first.large_items;
            let union2 = &second.small_items | &second.large_items;
            let union3 = &third.small_items | &third.large_items;
            let b = &(&union1 & &union2) & &union3;
            let badge = *b.iter().collect::<Vec<&u8>>().iter().nth(0).unwrap();
            score += score_item(*badge);
        }
        Ok(score)
	}
}

fn main() {
	let mut parser = LineParser::new(&rucksack_parser);
	let mut calculator = RucksackCalculator {};
	let status = Solution::solve::<Record, Answer>(&mut parser, &mut calculator);
	advent_exit(status);
}
