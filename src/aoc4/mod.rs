use std::fs;
use bit_set::BitSet;
use regex::Regex;

const INPUT: &str = "C:\\Users\\Abace\\IdeaProjects\\advent-of-code-2020\\src\\aoc4\\aoc4.txt";

struct Kv {
    key: u8,
    value: String
}

fn parse_file() -> Vec<Vec<Kv>> {
    fs::read_to_string(INPUT).unwrap().split("\r\n\r\n")
        .map(|x| x.split_whitespace()
            .map(|y| {
                let mut item = y.split(":");
                let key_str = item.next().unwrap();
                Kv {
                    key: match key_str {
                        "ecl" => 0,
                        "pid" => 1,
                        "eyr" => 2,
                        "hcl" => 3,
                        "byr" => 4,
                        "iyr" => 5,
                        "hgt" => 6,
                        "cid" => 7,
                        _ => panic!("Unknown field prefix: {}", key_str)
                    },
                    value: item.next().unwrap().parse().unwrap()
                }
            })
            .collect())
        .collect()
}

fn has_necessary_fields(item: &Vec<Kv>) -> bool {
    let mut set = BitSet::new();
    for item in item {
        set.insert(item.key as usize);
    }
    set.remove(7);
    match set.len() {
        7 => true,
        _ => false
    }
}

pub fn solve1() -> u32 {
    parse_file().iter()
        .filter(|x| has_necessary_fields(x))
        .count() as u32
}

pub fn solve2() -> u32 {
    let pid_regex = Regex::new(r"^\d{9}$").unwrap();
    let hcl_regex = Regex::new(r"^#[\da-f]{6}$").unwrap();
    parse_file().iter()
        .filter(|x| has_necessary_fields(x))
        .map(|x|
            x.iter()
                .map(|y|
                    match y.key {
                        0 => match y.value.as_str() {
                            "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
                            _ => false
                        }
                        1 => pid_regex.is_match(y.value.as_str()),
                        2 => match y.value.parse::<u32>() {
                            Ok(int) => match int {
                                2020 ..= 2030 => true,
                                _ => false
                            }
                            Err(_) => false
                        }
                        3 => hcl_regex.is_match(y.value.as_str()),
                        4 => match y.value.parse::<u32>() {
                            Ok(int) => match int {
                                1920 ..= 2002 => true,
                                _ => false
                            }
                            Err(_) => false
                        }
                        5 => match y.value.parse::<u32>() {
                            Ok(int) => match int {
                                2010 ..= 2020 => true,
                                _ => false
                            }
                            Err(_) => false
                        }
                        6 => {
                            let val = y.value.as_bytes();
                            match val.len() {
                                4 | 5 => {
                                    if val[val.len() - 2] == 'i' as u8
                                        && val[val.len() - 1] == 'n' as u8 {
                                        let num = y.value.split("in").next().unwrap();
                                        match num.parse::<u32>() {
                                            Ok(int) => match int {
                                                59 ..= 76 => true,
                                                _ => false
                                            }
                                            Err(_) => false
                                        }
                                    } else if val[val.len() - 2] == 'c' as u8
                                        && val[val.len() - 1] == 'm' as u8 {
                                        let num = y.value.split("cm").next().unwrap();
                                        match num.parse::<u32>() {
                                            Ok(int) => match int {
                                                150 ..= 193 => true,
                                                _ => false
                                            }
                                            Err(_) => false
                                        }
                                    } else {
                                        false
                                    }
                                }
                                _ => false
                            }
                        }
                        7 => true,
                        _ => panic!("Unknown field num: {}", y.key)
                    }
                )
                .filter(|y| !*y)
                .count() == 0
        )
        .filter(|x| *x)
        .count() as u32
}

fn main() {
    let val1 = solve1();
    println!("answer1: {}", val1);

    let val2 = solve2();
    println!("answer2: {}", val2)
}