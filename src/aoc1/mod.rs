use std::fs;

const INPUT: &str = "/home/bacef/IdeaProjects/untitled60/src/aoc1/aoc1.txt";

const TARGET: u32 = 2020;

pub struct Solver {
    pub data: Vec<u32>
}

impl Solver {

    pub fn new() -> Solver {
        let data = fs::read_to_string(INPUT).unwrap();
        let split_data: Vec<u32> = data.lines()
            .map(String::from)
            .map(|x| x.parse::<u32>().unwrap())
            .collect();

        Solver {
            data: split_data
        }
    }

    pub fn solve1(&self) -> Option<u32> {
        for i in 0 .. self.data.len() {
            for j in 0 .. self.data.len() {
                if &self.data[i] + &self.data[j] == TARGET {
                    return Some(&self.data[i] * &self.data[j])
                } else {
                    continue;
                };
            }
        }
        None
    }

    pub fn solve2(&self) -> Option<u32> {
        for i in 0 .. self.data.len() {
            for j in 0 .. self.data.len() {
                for k in 0 .. self.data.len() {
                    if &self.data[i] + &self.data[j] + &self.data[k] == TARGET {
                        return Some(&self.data[i] * &self.data[j] * &self.data[k])
                    } else {
                        continue;
                    };
                }
            }
        }
        None
    }
}
