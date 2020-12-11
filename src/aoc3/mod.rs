use std::fs;

const INPUT: &str = "C:\\Users\\Abace\\IdeaProjects\\advent-of-code-2020\\src\\aoc3\\aoc3.txt";

fn route1(vals: &Vec<&str>) -> u32 {
    let line_len = vals[0].len();
    let mut count = 0;

    for i in 0 .. vals.len() {
        if vals[i].as_bytes()[(i) % line_len] == ('#' as u8) {
            count += 1;
        }
    }

    count
}

fn route2(vals: &Vec<&str>) -> u32 {
    let line_len = vals[0].len();
    let mut count = 0;

    for i in 0 .. vals.len() {
        if vals[i].as_bytes()[(i * 3) % line_len] == ('#' as u8) {
            count += 1;
        }
    }

    count
}

fn route3(vals: &Vec<&str>) -> u32 {
    let line_len = vals[0].len();
    let mut count = 0;

    for i in 0 .. vals.len() {
        if vals[i].as_bytes()[(i * 5) % line_len] == ('#' as u8) {
            count += 1;
        }
    }

    count
}

fn route4(vals: &Vec<&str>) -> u32 {
    let line_len = vals[0].len();
    let mut count = 0;

    for i in 0 .. vals.len() {
        if vals[i].as_bytes()[(i * 7) % line_len] == ('#' as u8) {
            count += 1;
        }
    }

    count
}

fn route5(vals: &Vec<&str>) -> u32 {
    let line_len = vals[0].len();
    let mut count = 0;

    for i in 0 .. vals.len() {
        if i % 2 == 1 {
            continue
        }
        if vals[i].as_bytes()[(i / 2) % line_len] == ('#' as u8) {
            count += 1;
        }
    }

    count
}

pub fn solve1() -> u32 {
    let file = fs::read_to_string(INPUT).unwrap();
    let vals: Vec<&str> = file.lines().collect();

    route2(&vals)
}

pub fn solve2() -> u32 {
    let file = fs::read_to_string(INPUT).unwrap();
    let vals: Vec<&str> = file.lines().collect();

    let count1 = route1(&vals);
    println!("count 1: {}", count1);
    let count2 = route2(&vals);
    println!("count 2: {}", count2);
    let count3 = route3(&vals);
    println!("count 3: {}", count3);
    let count4 = route4(&vals);
    println!("count 4: {}", count4);
    let count5 = route5(&vals);
    println!("count 5: {}", count5);

    count1 * count2 * count3 * count4 * count5
}

fn main() {
    let val1 = solve1();
    println!("answer1: {}", val1);

    let val2 = solve2();
    println!("answer2: {}", val2)
}