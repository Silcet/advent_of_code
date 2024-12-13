use utils;

fn parse_input(input: &String) -> (Vec<i32>, Vec<i32>) {
    input.lines().fold(
        (
            Vec::with_capacity(input.len()),
            Vec::with_capacity(input.len()),
        ),
        |(mut left, mut right), line| {
            let numbers: Vec<i32> = line
                .split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect();
            if numbers.len() != 2 {
                panic!("Invalid input");
            }
            left.push(numbers[0]);
            right.push(numbers[1]);
            (left, right)
        },
    )
}

fn part_one(input: &String) -> i32 {
    let (mut left_col, mut right_col) = parse_input(input);

    left_col.sort();
    right_col.sort();
    left_col
        .iter()
        .zip(right_col.iter())
        .fold(0, |acc, (l, r)| acc + (l - r).abs())
}

fn part_two(input: &String) -> i32 {
    let (left_col, right_col) = parse_input(input);

    left_col
        .iter()
        .map(|l| right_col.iter().filter(|r| *r == l).count() as i32 * l)
        .sum()
}

fn main() {
    let mut aoc = utils::AdventOfCode::new(2024, 1);
    let input = aoc
        .get_input()
        .expect("Failed to get an input for the problem");

    let first_result = part_one(&input);
    println!("Part one result: {first_result}");

    let second_result = part_two(&input);
    println!("Part two result: {second_result}");
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "3   4
    4   3
    2   5
    1   3
    3   9
    3   3";
    const RESULT_ONE: i32 = 11;
    const RESULT_TWO: i32 = 31;

    #[test]
    fn part_one_test() {
        assert_eq!(part_one(&INPUT.to_string()), RESULT_ONE);
    }

    #[test]
    fn part_two_test() {
        assert_eq!(part_two(&INPUT.to_string()), RESULT_TWO);
    }
}
