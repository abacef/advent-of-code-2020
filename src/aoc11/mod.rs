use std::fs;

const INPUT: &str = "/home/bacef/IdeaProjects/untitled60/src/aoc11/aoc11.txt";

fn parse_input() -> Vec<Vec<Option<bool>>> {
    fs::read_to_string(INPUT).unwrap().lines()
        .map(|x| x.as_bytes().iter()
            .map(|y| match *y as char {
                'L' => Some(false),
                '.' => None,
                _ => panic!("Unknown character: {}", y)
            })
            .collect::<Vec<Option<bool>>>()
        )
        .collect::<Vec<Vec<Option<bool>>>>()
}

fn count_occupied_seats(board: Vec<Vec<Option<bool>>>) -> u32 {
    board.iter()
        .map(|x|
            x.iter()
                .filter(|y| match y {
                    Some(true) => true,
                    Some(false) => false,
                    None => false
                })
                .count()
        )
        .sum::<usize>() as u32
}

pub fn solve1() -> u32 {
    let start_board = parse_input();

    let mut board = start_board;

    let mut has_changed: bool = true;
    while has_changed {
        has_changed = false;
        let mut new_board = board.clone();
        for row in 0 .. board.len() {
            for col in 0 .. board[row].len() {
                match board[row][col] {
                    Some(x) => {
                        let mut occ_seats = 0;
                        for row_offset in 0 ..= 2 {
                            for col_offset in 0 ..= 2 {
                                let new_row: isize = row as isize + row_offset - 1;
                                let new_col: isize = col as isize + col_offset - 1;
                                if !(new_row == row as isize && new_col == col as isize) &&
                                    new_row >= 0 &&
                                    new_col >= 0 &&
                                    (new_row as usize) < board.len() &&
                                    (new_col as usize) < board[0].len() {
                                    // print!("checking: {},{} ", new_row, new_col);
                                    match board[new_row as usize][new_col as usize] {
                                        Some(y) => {
                                            match y {
                                                true => occ_seats += 1,
                                                false => ()
                                            }
                                        },
                                        None => ()
                                    };
                                }
                            }
                        }
                        // println!("{},{}:{}", row, col, occ_seats);
                        match x {
                            true => if occ_seats >= 4 {
                                new_board[row][col] = Some(false);
                                has_changed = true;
                            },
                            false => if occ_seats == 0 {
                                new_board[row][col] = Some(true);
                                has_changed = true;
                            }
                        }
                    },
                    None => continue
                }
            }
        }

        board = new_board;

        // for r in 0 .. board.len() {
        //     for c in 0 .. board[r].len() {
        //         print!("{}", match board[r][c] {
        //             Some(true) => "#",
        //             Some(false) => "L",
        //             None => "."
        //         })
        //     }
        //     println!()
        // }
        // println!();

    }

    count_occupied_seats(board)
}

pub fn solve2() -> u32 {
    let start_board = parse_input();

    let mut board = start_board;

    let mut has_changed: bool = true;
    while has_changed {
        has_changed = false;
        let mut new_board = board.clone();
        for row in 0 .. board.len() {
            for col in 0 .. board[row].len() {
                match board[row][col] {
                    Some(x) => {
                        let mut occ_seats = 0;
                        for row_offset in 0 ..= 2 {
                            for col_offset in 0 ..= 2 {
                                if !(row_offset == 1 && col_offset == 1) {
                                    let mut new_row: isize = row as isize;
                                    let mut new_col: isize = col as isize;

                                    loop {
                                        new_row = new_row as isize + row_offset - 1;
                                        new_col = new_col as isize + col_offset - 1;

                                        if new_row >= 0 &&
                                            new_col >= 0 &&
                                            (new_row as usize) < board.len() &&
                                            (new_col as usize) < board[0].len() {
                                            match board[new_row as usize][new_col as usize] {
                                                Some(y) => match y {
                                                    true => {
                                                        occ_seats += 1;
                                                        break;
                                                    },
                                                    false => break
                                                },
                                                None => continue
                                            }
                                        } else {
                                            break
                                        }
                                    }
                                }
                            }
                        }
                        // println!("{},{}:{}", row, col, occ_seats);
                        match x {
                            true => if occ_seats >= 5 {
                                new_board[row][col] = Some(false);
                                has_changed = true;
                            },
                            false => if occ_seats == 0 {
                                new_board[row][col] = Some(true);
                                has_changed = true;
                            }
                        }
                    },
                    None => continue
                }
            }
        }

        board = new_board;

        // for r in 0 .. board.len() {
        //     for c in 0 .. board[r].len() {
        //         print!("{}", match board[r][c] {
        //             Some(true) => "#",
        //             Some(false) => "L",
        //             None => "."
        //         })
        //     }
        //     println!()
        // }
        // println!();

    }

    count_occupied_seats(board)
}