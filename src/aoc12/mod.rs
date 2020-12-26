use std::fs;
use crate::aoc12::OpCode::{Vert, Hor, Rot, Forward};
use crate::aoc12::Facing::{North, South, East, West};

const INPUT: &str = "/home/bacef/IdeaProjects/untitled60/src/aoc12/aoc12.txt";

enum OpCode {
    Vert,
    Hor,
    Rot,
    Forward
}

struct Command {
    direction: OpCode,
    amount: i32
}

#[derive(Copy, Clone, Debug)]
enum Facing {
    North,
    East,
    South,
    West
}

impl Facing {
    pub fn from_int(i: i32) -> Facing {
        match i {
            0 => North,
            1 => East,
            2 => South,
            3 => West,
            _ => panic!("Unknown int for Facing to Int Conversion: {}", i)
        }
    }
}

#[derive(Debug)]
struct BoatState {
    facing: Facing,
    x: i32,
    y: i32
}
impl BoatState {
    pub fn new() -> BoatState {
        BoatState {
            facing: East,
            x: 0,
            y: 0
        }
    }
}

fn parse_input() -> Vec<Command> {
    fs::read_to_string(INPUT).unwrap().lines()
        .map(|x| {
            let bytes = x.as_bytes();
            let dir_char = bytes[0] as char;
            let parsed_amount = x[1 .. x.len()].parse::<i32>().unwrap();
            let amount = match dir_char {
                'S' | 'W' | 'L' => -parsed_amount,
                _ => parsed_amount
            };
            let dir = match dir_char {
                'N' | 'S' => Vert,
                'E' | 'W' => Hor,
                'L' | 'R' => Rot,
                'F' => Forward,
                _ => panic!("Unknown command: {}", dir_char)
            };

            Command {
                direction: dir,
                amount
            }
        })
        .collect()
}

pub fn solve1() -> u32 {
    let mut state = BoatState::new();

    let input = parse_input();
    for x in input {
        let right_twists = match x.direction {
            Rot => Some((x.amount / 90) as i32 + 4),
            _ => None
        };

        match x.direction {
            Vert => state.y += x.amount,
            Hor => state.x += x.amount,
            Rot => state.facing = Facing::from_int((state.facing as i32 + right_twists.unwrap()) % 4),
            Forward => match state.facing {
                North => state.y += x.amount,
                South => state.y -= x.amount,
                East => state.x += x.amount,
                West => state.x -= x.amount,
            }
        }
    }

    (state.x.abs() + state.y.abs()) as u32
}

#[derive(Debug)]
struct Waypoint {
    x: i32,
    y: i32
}
impl Waypoint {
    pub fn new() -> Waypoint {
        Waypoint {
            x: 10,
            y: 1
        }
    }
}

pub fn solve2() -> u32 {
    let mut boat_state = BoatState::new();
    let mut waypoint  = Waypoint::new();

    let input = parse_input();
    for x in input {
        // println!("{:?}", boat_state);
        // println!("{:?}", waypoint);
        // println!();

        let right_twists = match x.direction {
            Rot => Some(((x.amount / 90) as i32 + 4) % 4),
            _ => None
        };

        match x.direction {
            Vert => waypoint.y += x.amount,
            Hor => waypoint.x += x.amount,
            Rot => match right_twists.unwrap() {
                0 => (),
                1 => {
                    let fut_x = waypoint.y;
                    waypoint.y = -waypoint.x;
                    waypoint.x = fut_x;
                },
                2 => {
                    waypoint.x *= -1;
                    waypoint.y *= -1;
                },
                3 => {
                    let fut_x = -waypoint.y;
                    waypoint.y = waypoint.x;
                    waypoint.x = fut_x;
                }
                _ => panic!("Amount is not mod 4 for rot: {}", x.amount)
            },
            Forward => {
                boat_state.x += waypoint.x * x.amount;
                boat_state.y += waypoint.y * x.amount;
            },
        }
    }

    // println!("{:?}", boat_state);
    // println!("{:?}", waypoint);
    // println!();

    (boat_state.x.abs() + boat_state.y.abs()) as u32
}