mod advent;

use regex::Regex;
use advent::{advent_exit, Calculator, LineParser, Solution};

enum Move {
    Rock,
    Paper,
    Scissors,
    Invalid
}

impl Move {
    fn from_letter(letter: &str) -> Move {
        match letter {
            "A" => Move::Rock,
            "B" => Move::Paper,
            "C" => Move::Scissors,
            "X" => Move::Rock,
            "Y" => Move::Paper,
            "Z" => Move::Scissors,
            _ => Move::Invalid
        }
    }

    fn against(&self, other: &Move) -> Outcome {
        match (self, other) {
            (Move::Rock, Move::Rock) => Outcome::Draw,
            (Move::Rock, Move::Paper) => Outcome::Lose,
            (Move::Rock, Move::Scissors) => Outcome::Win,
            (Move::Paper, Move::Rock) => Outcome::Win,
            (Move::Paper, Move::Paper) => Outcome::Draw,
            (Move::Paper, Move::Scissors) => Outcome::Lose,
            (Move::Scissors, Move::Rock) => Outcome::Lose,
            (Move::Scissors, Move::Paper) => Outcome::Win,
            (Move::Scissors, Move::Scissors) => Outcome::Draw,
            _ => Outcome::Draw,
        }
    }

    fn points(&self) -> i64 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
            Move::Invalid => 0,
        }
    }
}

enum Outcome {
    Win,
    Lose,
    Draw,
    Invalid,
}

impl Outcome {
    fn points(&self) -> i64 {
        match self {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Lose => 0,
            Outcome::Invalid => 0,
        }
    }

    fn move_for_outcome(&self, against_move: &Move) -> Move {
        match (against_move, self) {
            (Move::Rock, Outcome::Lose) => Move::Scissors,
            (Move::Rock, Outcome::Draw) => Move::Rock,
            (Move::Rock, Outcome::Win) => Move::Paper,
            (Move::Paper, Outcome::Lose) => Move::Rock,
            (Move::Paper, Outcome::Draw) => Move::Paper,
            (Move::Paper, Outcome::Win) => Move::Scissors,
            (Move::Scissors, Outcome::Lose) => Move::Paper,
            (Move::Scissors, Outcome::Draw) => Move::Scissors,
            (Move::Scissors, Outcome::Win) => Move::Rock,
            _ => Move::Invalid,
        }
    }

    fn from_letter(s: &str) -> Outcome {
        match s {
            "X" => Outcome::Lose,
            "Y" => Outcome::Draw,
            "Z" => Outcome::Win,
            _ => Outcome::Invalid,
        }
    }
}

struct Record {
    opponent_move: Move,
    part_a_move: Move,
    part_b_outcome: Outcome,
}

impl Record {
    fn points(&self) -> i64 {
        self.part_a_move.points() + self.part_a_move.against(&self.opponent_move).points()
    }
}

struct ScoreCalculator {}
impl Calculator<Record, i64> for ScoreCalculator {
	fn solve_a(&mut self, records: &Vec<Record>) -> Result<i64, String> {
        let mut points : i64 = 0;
        for record in records {
            points += record.points();
        }
        Ok(points)
	}
	fn solve_b(&mut self, records: &Vec<Record>) -> Result<i64, String> {
        let mut points : i64 = 0;
        for record in records {
            let our_move = record.part_b_outcome.move_for_outcome(&record.opponent_move);
            points += our_move.points() + record.part_b_outcome.points();
        }

        Ok(points)
	}
}

fn rock_paper_scissors_line_parser(line_number: usize, s: &String) -> Result<Record, String> {
	let re = Regex::new(r"^([ABC]) ([XYZ])").unwrap();
    if let Some(cap) = re.captures(&s) {
        let first_letter = cap.get(1).unwrap().as_str();
        let second_letter = cap.get(2).unwrap().as_str();
        Ok(Record {
            opponent_move: Move::from_letter(first_letter),
            part_a_move: Move::from_letter(second_letter),
            part_b_outcome: Outcome::from_letter(second_letter),
        })
    } else {
        Err(format!("We found a line we didn't understand {}: '{}'", line_number, s)) 
    }
}

fn main() {
	let mut parser = LineParser::new(&rock_paper_scissors_line_parser);
	let mut calculator = ScoreCalculator {};
	let status = Solution::solve::<Record, i64>(&mut parser, &mut calculator);
	advent_exit(status);
}
