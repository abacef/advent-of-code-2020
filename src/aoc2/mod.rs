use std::fs;
use std::ops::Index;

const INPUT: &str = "/home/bacef/IdeaProjects/untitled60/src/aoc2/aoc2.txt";

pub struct Password {
    low_number: i32,
    high_number: i32,
    character: u8,
    password: String
}

fn read_input() -> Vec<Password> {
    fs::read_to_string(INPUT).unwrap().lines()
        .map(|x| x.split_whitespace())
        .map(|mut x| {
            let mut numbers = x.next().unwrap().split("-");
            Password {
                low_number: String::from(numbers.next().unwrap()).parse().unwrap(),
                high_number: String::from(numbers.next().unwrap()).parse().unwrap(),
                character: x.next().unwrap().as_bytes()[0],
                password: x.next().unwrap().to_string()
            }
        }).collect()
}

pub fn solve1() -> usize {
    read_input().iter()
        .map(|x| {
            let mut count = 0;
            for item in x.password.as_bytes() {
                if *item == x.character {
                    count += 1;
                }
            }
            count >= x.low_number && count <= x.high_number
        })
        .filter(|x| *x == true)
        .count()
}

pub fn solve2() -> usize {
    read_input().iter()
        .map(|x| {
            let pass_bytes = x.password.as_bytes();
            let low_char = pass_bytes[(x.low_number - 1) as usize];
            let high_char = pass_bytes[(x.high_number - 1) as usize];

            if (low_char == x.character) ^ (high_char == x.character) {
                true
            } else {
                false
            }
        })
        .filter(|x| *x == true)
        .count()
}

fn main() {
    let answer1 = solve1();
    println!("answer 1: {}", answer1);

    let answer2 = solve2();
    println!("answer 2: {}", answer2)
}