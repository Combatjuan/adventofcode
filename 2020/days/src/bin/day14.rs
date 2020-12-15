use std::env;
use std::fs;
use std::io::{BufRead, BufReader};
use std::collections::BTreeMap;
use std::process::exit;
use regex::Regex;

// SOME type
type Answer = (u64, u64);

#[derive(Debug)]
enum Data {
	Mask(String),
	Set(u64, u64),
    Invalid,
}

struct AddressIterator {
    // The mask we're using when we start this iteration
    // (it should not change over the course of its use)
    bits: [u8; 36],
    // The address whose values we need
    addr: u64,
    // The number of items we need to return which is equal to
    // 2 ^ (number of Xs in mask)
    permutations: u64,
    // Counts up until permutations
    p: u64
}

impl AddressIterator {
    fn new(mask: &String, addr: u64) -> AddressIterator {
        let x_count : u64 = mask.chars().filter(|x| *x == 'X').count() as u64;
        let mut bits : [u8; 36] = [0; 36];
        for (i, c) in mask.as_bytes().iter().enumerate() {
            bits[35 - i] = *c;
        }
        println!("NewIterator(addr: {}, Xs: {})", addr, (1 << x_count));
        AddressIterator {
            bits: bits,
            addr: addr,
            permutations: (1 << x_count),
            p: 0,
        }
    }
}

impl Iterator for AddressIterator {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        if self.p == self.permutations {
            None
        } else {
            let mut addr = self.addr;
            let mut xs = 0;
            for (i, c) in self.bits.iter().enumerate() {
                // Easier to reverse the number than reverse
                // the string.  :-/
                match c {
                    b'0' => {},
                    b'1' => {
                        addr = addr | (1 << i);
                    },
                    b'X' => {
                        let bit = (self.p >> xs) & 0x1;
                        xs += 1;
                        if bit != 0 {
                            addr = addr | (1 << i)
                        } else {
                            addr = addr & !(1 << i)
                        }
                    },
                    _ => {},
                }
            }
            self.p += 1;
            Some(addr)
        }
    }
}

fn masked_a(mask: &String, value: u64) -> u64 {
    let mut v = value;
    let base : u64 = 2;
    for (exp, c) in mask.chars().enumerate() {
        // Easier to reverse the number than reverse
        // the string.  :-/
        let exp = 35 - exp;
        match c {
            '0' => {
                v = v & (0xFFFFFFFFFFFFFFFF ^ base.pow(exp as u32));
            },
            '1' => {
                v = v | base.pow(exp as u32)
            },
            'X' => {},
            _ => {},
        }
    }
    v
}

fn calculate(data: &Vec<Data>) -> Result<Answer, &str> {
    // Part A
    let answer_a = {
        let mut mask = String::new();
        let mut memory : BTreeMap<u64, u64> = BTreeMap::new();
        for datum in data {
            match datum {
                Data::Mask(m) => { mask = m.clone() },
                Data::Set(addr, value) => {
                    let new_value = masked_a(&mask, *value);
                    memory.insert(*addr, new_value);
                },
                Data::Invalid => {}
            }
        }

        let mut total : u64 = 0;
        for num in memory.values() {
            total += num;
        }
        total
    };

    let answer_b = {
        let mut mask = String::new();
        let mut memory : BTreeMap<u64, u64> = BTreeMap::new();
        for datum in data {
            println!("Data: {:?}", datum);
            match datum {
                Data::Mask(m) => { mask = m.clone() },
                Data::Set(addr, value) => {
                    let addr_iter = AddressIterator::new(&mask, *addr);
                    for address in addr_iter {
                        println!("  Set {} to {}", address, *value);
                        memory.insert(address, *value);
                    }
                },
                Data::Invalid => {}
            }
        }
        let mut total : u64 = 0;
        for num in memory.values() {
            total += num;
        }
        total
    };
    Ok((answer_a, answer_b))
}

fn parse_line(line: &String) -> Data {
	let mask_re = Regex::new(r"^mask = ([01X]{36})").unwrap();
	let mem_re = Regex::new(r"^mem\[(\d+)\] = (\d+)").unwrap();
    if let Some(cap) = mask_re.captures(line) {
        Data::Mask(cap.get(1).unwrap().as_str().to_string())
    } else if let Some(cap) = mem_re.captures(line) {
        Data::Set(
            cap.get(1).unwrap().as_str().parse::<u64>().unwrap(),
            cap.get(2).unwrap().as_str().parse::<u64>().unwrap(),
        )
    } else {
        Data::Invalid
    }
}

fn load(filename: &str) -> Result<Vec<Data>, String> {
	if let Ok(file) = fs::File::open(filename) {
		let lines = BufReader::new(file).lines();
		let data : Vec<Data> = lines.map(|x| parse_line(&x.unwrap())).collect();
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

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn test_iterator() {
        let mask = "0000000000000000000000000000000000XX".to_string();
        let addr = 0x4;
        let mut sum = 0;
        println!("Start iter");
        for n in AddressIterator::new(&mask, addr) {
            println!("{}", n);
            sum += n;
        }
        println!("End iter");
        assert_eq!(22, sum);
    }
}
