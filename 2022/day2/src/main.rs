use std::cmp::Ordering;
use enum_iterator::{Sequence, next_cycle, previous_cycle};

#[derive(PartialEq, Sequence, Clone, Copy)]
enum RPS {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl RPS {
    fn game_value(&self, other: &Self) -> u32 {
        if self > other {
            6
        } else if self == other {
            3
        } else {
            0
        }
    }

    fn choose_outcome(&self, instruction: Instruction) -> Self {
        match instruction {
            Instruction::Lose => previous_cycle(self).unwrap(),
            Instruction::Draw => self.clone(),
            Instruction::Win => next_cycle(self).unwrap()
        }
    }
}

impl From<char> for RPS {
    fn from(shape: char) -> Self {
        match shape {
            'A' | 'X' => Self::Rock,
            'B' | 'Y' => Self::Paper,
            'C' | 'Z' => Self::Scissors,
            _ => panic!("Invalid shape!")
        }
    }
}

impl PartialOrd for RPS {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Self::Rock, Self::Scissors) => Some(Ordering::Greater),
            (Self::Rock, Self::Paper) => Some(Ordering::Less),
            (Self::Paper, Self::Rock) => Some(Ordering::Greater),
            (Self::Paper, Self::Scissors) => Some(Ordering::Less),
            (Self::Scissors, Self::Paper) => Some(Ordering::Greater),
            (Self::Scissors, Self::Rock) => Some(Ordering::Less),
            _ => Some(Ordering::Equal)
        }
    }
}

#[derive(Clone)]
enum Instruction {
    Lose = 0,
    Draw = 3,
    Win = 6
}

impl From<char> for Instruction {
    fn from(shape: char) -> Self {
        match shape {
            'X' => Self::Lose,
            'Y' => Self::Draw,
            'Z' => Self::Win,
            _ => panic!("Invalid shape!")
        }
    }
}

fn part_one(input: &String) -> u32 {
    let result: u32 = input
        .split('\n')
        .filter_map(|line| {
            if line.len() == 0 {
                return None;
            }
            let shapes: Vec<RPS> = line.chars().filter(|c| !c.is_whitespace()).map(|c| RPS::from(c)).collect();
            
            Some(shapes[1].game_value(&shapes[0]) + shapes[1] as u32)
        })
        .sum();

    result
}

fn part_two(input: &String) -> u32 {
    let result: u32 = input
        .split('\n')
        .filter_map(|line| {
            if line.len() == 0 {
                return None;
            }
            let shapes: Vec<char> = line.chars().filter(|c| !c.is_whitespace()).collect();
            let oponent = RPS::from(shapes[0]);
            let instruction = Instruction::from(shapes[1]);
            
            Some(instruction.clone() as u32 + oponent.choose_outcome(instruction) as u32)
        })
        .sum();

    result
}

fn main() {
    let input = utils::get_input("https://adventofcode.com/2022/day/2/input".to_string()).unwrap();

    let result_one = part_one(&input);
    println!("Part one result: {}", result_one);

    let result_two = part_two(&input);
    println!("Part two result: {}", result_two);
}

#[cfg(test)]
mod tests {
    use crate::part_one;
    use crate::part_two;

    #[test]
    fn part_one_example() {
        let input = "A Y
B X
C Z".to_string();
        let result = part_one(&input);
        assert_eq!(result, 15u32);
    }

    #[test]
    fn part_two_example() {
        let input = "A Y
B X
C Z".to_string();
        let result = part_two(&input);
        assert_eq!(result, 12u32);
    }
}

