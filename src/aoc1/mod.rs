use std::fs;
use std::time::Instant;

const INPUT: &str = "/home/bacef/IdeaProjects/untitled60/src/aoc1/aoc1.txt";

const TARGET: u32 = 2020;

pub struct Solver {
    pub data: Vec<u32>
}

impl Solver {

    pub fn new() -> Solver {
        Solver {
            data: fs::read_to_string(INPUT).unwrap().lines()
                .map(String::from)
                .map(|x| x.parse::<u32>().unwrap())
                .collect()
        }
    }

    fn build_bins(&self) -> Vec<Vec<u32>> {
        let mut bins: Vec<Vec<u32>> = Vec::with_capacity(10);
        for _ in 0 .. 10 {
            bins.push(Vec::new());
        }

        for i in 0 .. self.data.len() {
            bins[(&self.data[i] % 10) as usize].push(self.data[i]);
        }

        bins
    }

    /// Bin each number based on its last digit.
    /// If one of the numbers ends in zero, the other does as well. If it is a 9, the other is a 1
    /// and so on.
    pub fn solve1(&self) -> Option<u32> {
        let bins: Vec<Vec<u32>> = self.build_bins();

        for i in 0 .. 6 {
            let op_i = (10 - i) % 10;
            for j in 0 .. bins[i].len() {
                for k in 0 .. bins[op_i].len() {
                    if !(i == op_i && j != k) {
                        let num1 = bins[i][j];
                        let num2 = bins[op_i][k];
                        if num1 + num2 == TARGET {
                            return Some(bins[i][j] * bins[op_i][k])
                        } else {
                            continue;
                        }
                    }
                }
            }
        }
        None
    }

    pub fn solve2(&self) -> Option<u32> {
        let bins: Vec<Vec<u32>> = self.build_bins();

        for i in 0 .. self.data.len() {
            for j in 0 .. self.data.len() {
                if i != j {
                    let num1_plus_num2 = self.data[i] + self.data[j];
                    let opt_mod = num1_plus_num2 % 10;
                    let bin_num = ((10 - opt_mod) % 10) as usize;

                    for k in 0 .. bins[bin_num].len() {
                        let third_num = bins[bin_num][k];
                        if num1_plus_num2 + third_num == TARGET {
                            return Some(&self.data[i] * &self.data[j] * third_num)
                        } else {
                            continue;
                        };
                    }
                }
            }
        }
        None
    }

    /// Brute force check every pairwise number
    pub fn old_solve1(&self) -> Option<u32> {
        for i in 0 .. self.data.len() {
            for j in 0 .. self.data.len() {
                if i != j {
                    if &self.data[i] + &self.data[j] == TARGET {
                        return Some(&self.data[i] * &self.data[j])
                    } else {
                        continue;
                    };
                }
            }
        }
        None
    }

    /// Brute force check every triwise number
    pub fn old_solve2(&self) -> Option<u32> {
        for i in 0 .. self.data.len() {
            for j in 0 .. self.data.len() {
                for k in 0 .. self.data.len() {
                    if i != j && i != k && j != k {
                        if &self.data[i] + &self.data[j] + &self.data[k] == TARGET {
                            return Some(&self.data[i] * &self.data[j] * &self.data[k])
                        } else {
                            continue;
                        };
                    }
                }
            }
        }
        None
    }
}


const ERROR_MESSAGE: &str = "ERROR! Not found!";

fn display_message(val: Option<u32>) -> String {
    match val {
        Some(val) => val.to_string(),
        None => ERROR_MESSAGE.to_string()
    }
}

fn main() {
    let solver = Solver::new();

    let old_now1 = Instant::now();
    let old1 = solver.old_solve1();
    println!("old part 1 time: {}", old_now1.elapsed().as_micros());

    let message1_old = display_message(old1);
    println!("old part 1 answer: {}", message1_old);


    let new_now1 = Instant::now();
    let new1 = solver.solve1();
    println!("new part 1 time: {}", new_now1.elapsed().as_micros());

    let message1_new = display_message(new1);
    println!("old part 1 answer: {}", message1_new);



    let old_now2 = Instant::now();
    let old2 = solver.old_solve2();
    println!("old part 2 time: {}", old_now2.elapsed().as_micros());

    let message2_new = display_message(old2);
    println!("old part 2 answer: {}", message2_new);


    let new_now2 = Instant::now();
    let new2 = solver.solve2();
    println!("new part 2 time: {}", new_now2.elapsed().as_micros());

    let message2_new = display_message(new2);
    println!("new part 2 answer: {}", message2_new);
}
