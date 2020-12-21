use std::fs;

const INPUT: &str = "/home/bacef/IdeaProjects/untitled60/src/aoc10/aoc10.txt";

const TRIFIB: [u128; 5] = [1, 1, 2, 4, 7];

fn parse_input() -> Vec<u32> {
    let mut ret = fs::read_to_string(INPUT).unwrap().lines()
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    ret.sort();
    ret
}

pub fn solve1() -> u32 {
    let items = parse_input();

    let mut count1 = 0;
    let mut count3 = 1; // for the last jump
    match items[0] {
        1 => count1 += 1,
        3 => count3 += 1,
        _ => ()
    }
    for i in 0 .. items.len() - 1 {
        match items[i + 1] - items[i] {
            1 => count1 += 1,
            3 => count3 += 1,
            _ => ()
        }
    }
    count1 * count3
}

fn bf(items: &Vec<u32>) -> Vec<u32> {
    let mut new_items: Vec<u32> = Vec::new();
    let mut count = 0;
    for i in 0 .. items.len() - 1 {
        count += 1;
        if items[i + 1] - items[i] == 3 {
            new_items.push(count);
            count = 0;
        }
    }
    println!("{:?}", new_items);
    new_items
}

pub fn solve2() -> u128 {
    let mut items = parse_input();
    items.push(0);
    items.push(*(&items.iter().max().unwrap()) + 3);
    items.sort();

    println!("{:?}", items);
    let nums = bf(&items);
    let res = nums.iter().map(|x| TRIFIB[(x - 1) as usize]).product::<u128>();
    res
}