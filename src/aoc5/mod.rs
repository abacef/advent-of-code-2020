use std::fs;
use bit_set::BitSet;

const INPUT: &str = "C:\\Users\\Abace\\IdeaProjects\\advent-of-code-2020\\src\\aoc5\\aoc5.txt";

fn solve(input: &str) -> u32 {
    let mut front = 0;
    let mut back = 127;
    let mut l = 0;
    let mut r = 7;
    input.as_bytes().iter().for_each(|x| match *x as char {
        'F' => {
            back = ((back - front) / 2) + front;
        }
        'B' => {
            front = ((back - front) / 2) + front + 1
        }
        'R' => {
            l = ((r - l) / 2) + l + 1
        }
        'L' => {
            r = ((r - l) / 2) + l
        }
        _ => panic!("Unknown input: {}", x)
    });
    front * 8 + l
}

pub fn solve1() -> u32 {
    fs::read_to_string(INPUT).unwrap().lines()
        .map(|x| solve(x))
        .max().unwrap()
}

pub fn solve2() -> u32 {
    let mut bs = BitSet::with_capacity(926);
    fs::read_to_string(INPUT).unwrap().lines()
        .map(|x| solve(x))
        .for_each(|x| {
            bs.insert(x as usize);
            ()
        });

    let mut is_run = true;
    for i in 0..926 {
        if !bs.contains(i) {
            if !is_run {
                println!("{}", i);
            }
        } else {
            return i as u32;
        }
    }

    0
}
