#[macro_use]
extern crate lazy_static;

use std::env;
use std::fs;
use std::io::{BufRead, BufReader};
use std::process::exit;
use regex::Regex;

// SOME type
type Answer = (i64, i64);

#[derive(Debug)]
struct Data {
	byr: Option<i32>,
	iyr: Option<i32>,
	eyr: Option<i32>,
	hgt: Option<i32>,
	hgt_unit: Option<String>,
	hcl: Option<String>,
	ecl: Option<String>,
	pid: Option<String>,
	cid: Option<String>,
}

fn is_valid_a(license: &Data) -> Option<&Data> {
	match license {
		Data {
			byr: Some(_), iyr: Some(_), eyr: Some(_), hgt: Some(_), hgt_unit: Some(_),
			hcl: Some(_), ecl: Some(_), pid: Some(_), cid: _
		} => Some(license),
		_ => None,
	}
}

fn check_byr(byr: i32) -> bool {
	if byr >= 1920 && byr <= 2002 {
		true
	} else {
		println!("byr {} outside range", byr);
		false
	}
}

fn check_iyr(iyr: i32) -> bool {
	if iyr >= 2010 && iyr <= 2020 {
		true
	} else {
		println!("iyr {} outside range", iyr);
		false
	}
}

fn check_eyr(eyr: i32) -> bool {
	if eyr >= 2020 && eyr <= 2030 {
		true
	} else {
		println!("eyr {} outside range", eyr);
		false
	}
}

fn check_hgt(hgt: i32, hgt_unit: &String) -> bool {
	if !["cm", "in"].contains(&hgt_unit.as_str()) {
		println!("hgt_unit invalid: {}", hgt_unit);
		false
	} else if hgt_unit == "cm" && hgt >= 150 && hgt <= 193
		|| hgt_unit == "in" && hgt >= 59 && hgt <= 76 {
		true
	} else {
		println!("hgt out of range: {}{}", hgt, hgt_unit);
		false
	}
}

fn check_hcl(hcl: &str) -> bool {
	lazy_static! {
		static ref RE : Regex = Regex::new(r"^#[0-9a-f]{6}").unwrap();
	}
	if let Some(_) = RE.find(&hcl) {
		true
	} else {
		println!("hcl invalid: {}", hcl);
		false
	}
}

fn check_ecl(ecl: &str) -> bool {
	if ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&ecl) {
		true
	} else {
		println!("ecl unrecognized: {}", ecl);
		false
	}
}

fn check_pid(pid: &str) -> bool {
	if pid.len() != 9 {
		println!("pid has wrong length: {}", pid);
		false
	} else if !pid.chars().all(|c| ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'].contains(&c)) {
		println!("pid has invalid chars: {}", pid);
		false
	} else {
		true
	}
}

fn is_valid_b(license: &Data) -> Option<&Data> {
	match license {
		Data {
			byr: Some(byr), iyr: Some(iyr), eyr: Some(eyr), hgt: Some(hgt), hgt_unit: Some(hgt_unit),
			hcl: Some(hcl), ecl: Some(ecl), pid: Some(pid), cid: _
		} if check_byr(*byr)
			&& check_iyr(*iyr)
			&& check_eyr(*eyr)
			&& check_hgt(*hgt, hgt_unit)
			&& check_hcl(hcl.as_str())
			&& check_ecl(ecl.as_str())
			&& check_pid(pid.as_str())
		=> Some(license),
		_ => None,
	}
}

fn calculate(data: &Vec<Data>) -> Result<Answer, &str> {
	let mut count_a : i64 = 0;
	let mut count_b : i64 = 0;
	for license in data {
		if let Some(_) = is_valid_a(&license) {
			count_a += 1;
		}

		if let Some(license) = is_valid_b(&license) {
			println!("B Valid: {:?}", license);
			count_b += 1;
		} else {
			println!("B Invalid: {:?}", license);
		}
		println!("");
	}
	Ok((count_a, count_b))

	//Ok(data.iter().filter_map(|x| is_valid(&x)).count() as i64)
}

fn height_number(s: &String) -> i32 {
	let num : String
		= s.chars().filter(|c| ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'].contains(c)).collect();
	num.parse::<i32>().unwrap()
}

fn height_unit(s: &String) -> String {
	let unit : String = s.chars().filter(|c| !['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'].contains(c)).collect();
	unit
}

fn parse(lines: &mut dyn Iterator<Item=Result<String, std::io::Error>>) -> Option<Data> {
	let mut byr: Option<i32> = None;
	let mut iyr: Option<i32> = None;
	let mut eyr: Option<i32> = None;
	let mut hgt: Option<i32> = None;
	let mut hgt_unit: Option<String> = None;
	let mut hcl: Option<String> = None;
	let mut ecl: Option<String> = None;
	let mut pid: Option<String> = None;
	let mut cid: Option<String> = None;
	let re = Regex::new(r"([[:alpha:]]+:\S+)").unwrap();
	let mut is_record = false;
	loop {
		let next_line = lines.next();
		let line = match next_line {
			None => { break },
			Some(Ok(l)) if l.is_empty() => { break },
			Some(Err(_)) => { break },
			Some(Ok(l)) => l,
		};

		for pair in re.captures_iter(line.as_str()) {
			is_record = true;
			for p in 0..pair.len() - 1 {
				let mut parts = pair.get(p).unwrap().as_str().split(":");
				let key = parts.next().unwrap();
				match key {
					"byr" => { byr = Some(parts.next().unwrap().to_string().parse::<i32>().unwrap()) },
					"iyr" => { iyr = Some(parts.next().unwrap().to_string().parse::<i32>().unwrap()) },
					"eyr" => { eyr = Some(parts.next().unwrap().to_string().parse::<i32>().unwrap()) },
					"hgt" => {
						let h = parts.next().unwrap().to_string();
						hgt = Some(height_number(&h));
						hgt_unit = Some(height_unit(&h));
					},
					"hcl" => { hcl = Some(parts.next().unwrap().to_string()) },
					"ecl" => { ecl = Some(parts.next().unwrap().to_string()) },
					"pid" => { pid = Some(parts.next().unwrap().to_string()) },
					"cid" => { cid = Some(parts.next().unwrap().to_string()) },
					_ => {}
				}
			}
		}
	}
	match is_record {
		true => Some(Data {
			byr,
			iyr,
			eyr,
			hgt,
			hgt_unit,
			hcl,
			ecl,
			pid,
			cid
		}),
		false => None
	}
}

fn load(filename: &str) -> Result<Vec<Data>, String> {
	if let Ok(file) = fs::File::open(filename) {
		let mut lines : &mut dyn Iterator<Item=Result<String, std::io::Error>> = &mut BufReader::new(file).lines();
		let mut data : Vec<Data> = vec![];
		while let Some(record) = parse(&mut lines) {
			data.push(record);
		}
		match data.is_empty() {
			true => Err(format!("Failed to parse every line in {}", filename)),
			false => Ok(data),
		}
	} else {
		Err(format!("Could not load data from '{}'.", filename))
	}
}

fn run(args: &Vec<String>) -> Result<Answer, String> {
	if args.len() > 1 {
		let filename = &args[1];
		match load(filename) {
			Ok(data) => {
				let answer = calculate(&data);
				if let Ok(a) = answer {
					Ok(a)
				} else if let Err(s) = answer {
					Err(format!("Could not find a solution: {} ", s))
				} else {
					Err(String::from("This branch is not reachable"))
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
		Ok((a, b)) => {
			println!("Answer A: {}", a);
			println!("Answer B: {}", b);
			0
		},
		Err(err) => {
			println!("{}", err);
			1
		}
	});
}

