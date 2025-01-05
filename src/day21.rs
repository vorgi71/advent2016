mod utils;

use regex::Regex;
use std::ops::{Range, RangeInclusive};
use utils::utils::read_lines;

#[derive(Debug)]
enum Instruction {
    SwapPosition(usize, usize),
    SwapLetter(char, char),
    ReversePositions(usize, usize),
    RotateLeft(usize),
    RotateRight(usize),
    RotateBasedOnLetter(char),
    MovePosition(usize, usize),
}

fn parse_instruction(line: &str) -> Option<Instruction> {
    let swap_pos_re = Regex::new(r"^swap position (\d+) with position (\d+)$").unwrap();
    let swap_letter_re = Regex::new(r"^swap letter (\w) with letter (\w)$").unwrap();
    let reverse_re = Regex::new(r"^reverse positions (\d+) through (\d+)$").unwrap();
    let rotate_left_re = Regex::new(r"^rotate left (\d+) step(?:s)?$").unwrap();
    let rotate_right_re = Regex::new(r"^rotate right (\d+) step(?:s)?$").unwrap();
    let rotate_based_re = Regex::new(r"^rotate based on position of letter (\w)$").unwrap();
    let move_pos_re = Regex::new(r"^move position (\d+) to position (\d+)$").unwrap();

    if let Some(captures) = swap_pos_re.captures(line) {
        Some(Instruction::SwapPosition(
            captures[1].parse().ok()?,
            captures[2].parse().ok()?,
        ))
    } else if let Some(captures) = swap_letter_re.captures(line) {
        Some(Instruction::SwapLetter(
            captures[1].chars().next()?,
            captures[2].chars().next()?,
        ))
    } else if let Some(captures) = reverse_re.captures(line) {
        Some(Instruction::ReversePositions(
            captures[1].parse().ok()?,
            captures[2].parse().ok()?,
        ))
    } else if let Some(captures) = rotate_left_re.captures(line) {
        Some(Instruction::RotateLeft(captures[1].parse().ok()?))
    } else if let Some(captures) = rotate_right_re.captures(line) {
        Some(Instruction::RotateRight(captures[1].parse().ok()?))
    } else if let Some(captures) = rotate_based_re.captures(line) {
        Some(Instruction::RotateBasedOnLetter(
            captures[1].chars().next()?,
        ))
    } else if let Some(captures) = move_pos_re.captures(line) {
        Some(Instruction::MovePosition(
            captures[1].parse().ok()?,
            captures[2].parse().ok()?,
        ))
    } else {
        None
    }
}

fn main() {
    let input1 = read_lines("data/day21_1.txt");
    let result1 = execute_instructions(&input1, "abcde");
    println!("Result1: {}", result1);
    
    let input2 = read_lines("data/day21_2.txt");
    let result2 = execute_instructions(&input2, "abcdefgh");
    println!("Result2: {}", result2);
}

fn execute_instructions(input: &[String], code: &str) -> String {
    let instructions: Vec<Instruction> = input
        .iter()
        .filter_map(|line| parse_instruction(line))
        .collect();

    let mut code_array: Vec<char> = code.chars().collect();

    instructions.iter().for_each(|instruction| {
        match instruction {
            Instruction::SwapPosition(from, to) => {
                code_array.swap(*from, *to);
            }
            Instruction::SwapLetter(from_char, to_char) => {
                let from = code_array.iter().position(|c| c == from_char).unwrap();
                let to = code_array.iter().position(|c| c == to_char).unwrap();
                code_array.swap(from, to);
            }
            Instruction::ReversePositions(from, to) => {
                let mut helper: Vec<char> = code_array[*from..=*to].to_vec();
                helper.reverse();

                for (i, &c) in helper.iter().enumerate() {
                    code_array[from + i] = c;
                }
            }
            Instruction::RotateLeft(amount) => {
                code_array.rotate_left(*amount);
            }
            Instruction::RotateRight(amount) => {
                code_array.rotate_right(*amount);
            }
            Instruction::RotateBasedOnLetter(from_char) => {
                let mut amount = code_array.iter().position(|c| c == from_char).unwrap();
                if amount >= 4 {
                    amount += 2;
                } else {
                    amount += 1;
                }
                amount%=code_array.len();
                code_array.rotate_right(amount);
            }
            Instruction::MovePosition(from, to) => {
                let helper = code_array[*from];
                code_array.remove(*from);
                code_array.insert(*to, helper);
            }
        }
        println!("{}", code_array.iter().collect::<String>());
    });

    code_array.iter().collect()
}
