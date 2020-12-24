use std::fs;
use crate::aoc12::OpCode::{North, South, East, West, Left, Right, Forward};

const INPUT: &str = "/home/bacef/IdeaProjects/untitled60/src/aoc12/test1.txt";

enum OpCode {
    North,
    South,
    East,
    West,
    Left,
    Right,
    Forward
}

struct Command {
    direction: OpCode,
    amount: u32
}

#[derive(Copy, Clone)]
enum Facing {
    North,
    East,
    South,
    West
}

impl Facing {
    pub fn from_int(i: u32) -> Facing {
        match i {
            0 => Facing::North,
            1 => Facing::East,
            2 => Facing::South,
            3 => Facing::West,
            _ => panic!("Unknown int for Facing to Int Converison: {}", i)
        }
    }
}

struct BoatState {
    facing: Facing,
    x: i32,
    y: i32
}

pub fn solve1() -> u32 {
    let mut state = BoatState {
        facing: Facing::East,
        x: 0,
        y: 0
    };

    fs::read_to_string(INPUT).unwrap().lines()
        .map(|x| {
            let bytes = x.as_bytes();
            let dir = bytes[0] as char;
            let am = x[1 .. x.len()].parse::<u32>().unwrap();
            Command {
                direction: match dir {
                    'N' => North,
                    'S' => South,
                    'E' => East,
                    'W' => West,
                    'L' => Left,
                    'R' => Right,
                    'F' => Forward,
                    _ => panic!("Unknown command: {}", dir)
                },
                amount: am
            }
        })
        .for_each(|x| {
            let right_twists = match x.direction {
                Right | Left => Some(match x.amount {
                    90 => 1,
                    180 => 2,
                    270 => 3,
                    _ => panic!("Unknown amount for r or l: {}", x.amount)
                }),
                _ => None
            };

            match x.direction {
                North => state.y += x.amount as i32,
                South => state.y -= x.amount as i32,
                East => state.x += x.amount as i32,
                West => state.x -= x.amount as i32,
                Right => state.facing = Facing::from_int(((state.facing as i32 + right_twists.unwrap()) % 4) as u32),
                Left => state.facing = Facing::from_int(((state.facing as i32 + -right_twists.unwrap()) % 4) as u32),
                Forward => match state.facing {
                    Facing::North => state.y += x.amount as i32,
                    Facing::South => state.y -= x.amount as i32,
                    Facing::East => state.x += x.amount as i32,
                    Facing::West => state.x -= x.amount as i32,
                }
            }
        });

    (state.x.abs() + state.y.abs()) as u32

}

pub fn solve2() -> u32 {
    2
}