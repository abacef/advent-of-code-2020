use std::time::Instant;

mod aoc1;

const ERROR_MESSAGE: &str = "ERROR! Not found!";

fn main() {
    let solver = aoc1::Solver::new();

    let old_now = Instant::now();
    let _ = solver.old_solve1();
    println!("old part 1 time: {}", old_now.elapsed().as_micros());

    let new_now = Instant::now();
    let res1 = solver.solve1();
    println!("new part 1 time: {}", new_now.elapsed().as_micros());

    let message1 = match res1 {
        Some(val) => val.to_string(),
        None => ERROR_MESSAGE.to_string()
    };
    println!("Part 1 answer: {}", message1);

    let now2 = Instant::now();
    let res2 = solver.solve2();
    println!("part 2 time: {}", now2.elapsed().as_micros());

    let message2 = match res2 {
        Some(val) => val.to_string(),
        None => ERROR_MESSAGE.to_string()
    };
    println!("Part 2 answer: {}", message2);
}
