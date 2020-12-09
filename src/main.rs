use std::time::Instant;

mod aoc1;

const ERROR_MESSAGE: &str = "ERROR! Not found!";

fn display_message(val: Option<u32>) -> String {
    match val {
        Some(val) => val.to_string(),
        None => ERROR_MESSAGE.to_string()
    }
}

fn main() {
    let solver = aoc1::Solver::new();

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
