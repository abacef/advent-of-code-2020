mod aoc1;

const ERROR_MESSAGE: &str = "ERROR! Not found!";

fn main() {
    let solver = aoc1::Solver::new();

    let res1 = solver.solve1();
    let message1 = match res1 {
        Some(val) => val.to_string(),
        None => ERROR_MESSAGE.to_string()
    };
    println!("Part 1 answer: {}", message1);

    let res2 = solver.solve2();
    let message2 = match res2 {
        Some(val) => val.to_string(),
        None => ERROR_MESSAGE.to_string()
    };
    println!("Part 2 answer: {}", message2);
}
