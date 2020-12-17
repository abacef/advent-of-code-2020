use std::fs;
use std::collections::HashSet;
use bit_set::BitSet;
use reduce::Reduce;

const INPUT: &str = "/home/bacef/IdeaProjects/untitled60/src/aoc6/aoc6.txt";

pub fn solve1() -> usize {
    fs::read_to_string(INPUT).unwrap().split("\n\n")
        .map(|x| x.lines().flat_map(|y| y.as_bytes()))
        .map(|x| {
            // if only I could x.collect::<HashSet<u8>>()
            let mut set: HashSet<u8> = HashSet::new();
            for y in x {
                set.insert(*y);
            }
            set.len()
        })
        .sum()
}

pub fn solve2() -> usize {
    fs::read_to_string(INPUT).unwrap().split("\n\n")
        .map(|x|
            // if only I could x.lines().collect::<HashSet<u8>>()
            x.lines().map(|y| {
                let mut set = BitSet::new();
                for z in y.as_bytes() {
                    set.insert(*z as usize);
                }
                set
            })
        )
        .map(|x|
            x.reduce(|mut a, b| {
                a.intersect_with(&b);
                a
            }).unwrap().len()
        )
        .sum()
}

fn main() {
    println!("{}", solve1());
    println!("{}", solve2());
}