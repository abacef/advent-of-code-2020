use std::fs;

const INPUT: &str = "/home/bacef/IdeaProjects/untitled60/src/aoc2/aoc2.txt";

pub struct Password {
    low_number: i32,
    high_number: i32,
    character: u8,
    password: String
}

pub fn solve1() -> usize {
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
        })
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