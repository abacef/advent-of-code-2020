use std::fs;
use bit_set::BitSet;
use crate::aoc8::Instruction::{Jmp, Nop, Acc};

const INPUT: &str = "/home/bacef/IdeaProjects/untitled60/src/aoc8/aoc8.txt";

#[derive(Clone)]
enum Instruction {
    Acc(i32),
    Jmp(i32),
    Nop(i32)
}

fn parse_input() -> Vec<Instruction> {
    fs::read_to_string(INPUT).unwrap().lines()
        .map(|x| x.split_whitespace().collect::<Vec<&str>>())
        .map(|mut x| match x.len() {
            2 => x,
            _ => panic!("Incorrect instruction: {:?}", x)
        })
        .map(|x| {
            let num = x[1].parse::<i32>().unwrap();
            match x[0] {
                "jmp" => Instruction::Jmp(num),
                "nop" => Instruction::Nop(num),
                "acc" => Instruction::Acc(num),
                _ => panic!("Incorrect instruction name {}", x[0])
            }
        })
        .collect()
}

fn has_inf_looped(instructions: &Vec<Instruction>) -> (bool, i32) {
    let mut has_run = BitSet::with_capacity(instructions.len());

    let mut p_counter: i32= 0;
    let mut accumulator = 0;
    while !has_run.contains(p_counter as usize) {
        let new_inst = instructions.get(p_counter as usize);
        match new_inst {
            Some(new_inst) => {
                has_run.insert(p_counter as usize);
                match new_inst {
                    Instruction::Acc(x) => {
                        accumulator += x;
                        p_counter += 1;
                    },
                    Instruction::Jmp(x) =>
                        p_counter += x,
                    Instruction::Nop(_) =>
                        p_counter += 1
                }
            },
            None => return (false, accumulator)
        }

    }
    (true, accumulator)
}

pub fn solve1() -> i32 {
    let instructions = parse_input();

    let (_, acc) = has_inf_looped(&instructions);
    acc
}

pub fn solve2() -> i32 {
    let mut instructions = parse_input();

    for i in 0 .. instructions.len() {
        let old_inst = instructions[i].clone();
        let new_inst = match old_inst {
            Acc(_) => None,
            Nop(x) => Some(Jmp(x)),
            Jmp(x) => Some(Nop(x))
        };

        match new_inst {
            None => continue,
            Some(x) => {
                instructions[i] = x;
                let (looped, acc) = has_inf_looped(&instructions);
                match looped {
                    true => instructions[i] = old_inst,
                    false => return acc
                }
            }
        }
    }
    -1
}