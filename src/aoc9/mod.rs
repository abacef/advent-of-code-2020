use std::fs;

const INPUT: &str = "/home/bacef/IdeaProjects/untitled60/src/aoc9/aoc9.txt";
const NUMS_BEFORE: usize = 25;

fn parse_input() -> Vec<u64> {
    fs::read_to_string(INPUT).unwrap().lines()
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<u64>>()
}

pub fn solve1() -> u64 {
    let items = parse_input();

    'label: for i in NUMS_BEFORE .. items.len() {
        for j in 1 ..= NUMS_BEFORE {
            for k in 1 ..= NUMS_BEFORE {
                if items[i - j] + items[i - k] == items[i] {
                    continue 'label
                }
            }
        }
        return items[i]
    }
    0
}

// brute force
pub fn solve2() -> u64 {
    let items = parse_input();

    let bad_num = solve1();

    for i in 0 .. items.len() {
        let mut sum = 0;
        let mut next_num = i;
        while (sum < bad_num) || (next_num - i == 0) {
            sum += items[next_num];
            next_num += 1;
        }
        if sum == bad_num {
            let mut max = 0;
            let mut min = u64::MAX;
            for j in i .. next_num {
                let item = items[j];
                if item < min {
                    min = item;
                }
                if item > max {
                    max = item;
                }
            }
            return max + min
        }
    }
    3
}