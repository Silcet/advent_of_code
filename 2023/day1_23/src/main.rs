use regex::Regex;
use std::collections::HashMap;
use utils;

fn part_one(input: &String) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut numbers = line.chars().filter_map(|c| c.to_digit(10));
            let first = numbers.next().unwrap_or(0);
            let second = numbers.last().unwrap_or(first);
            first * 10 + second
        })
        .sum()
}

fn slice_to_number(slice: &str) -> u32 {
    let digit_map: HashMap<&str, u32> = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    match slice.parse::<u32>() {
        Ok(digit) => digit,
        Err(_) => *digit_map.get(slice).unwrap_or(&0),
    }
}

fn part_two(input: &String) -> u32 {
    let digit_regex: Regex =
        Regex::new(r"\d|one|two|three|four|five|six|seven|eight|nine").unwrap();

    input
        .lines()
        .inspect(|l| println!("{l}"))
        .map(|line| {
            let mut first: Option<u32> = None;
            let mut second: Option<u32> = None;

            for i in 1..=line.len() {
                if first.is_none() {
                    let first_slice = &line[0..i];
                    println!("First slice: {first_slice:?}");
                    if let Some(first_match) = digit_regex.find(&line[0..i]) {
                        first = Some(slice_to_number(first_match.as_str()));
                        println!("First: {}", first.unwrap());
                    }
                }
                if second.is_none() {
                    let second_slice = &line[line.len() - i..line.len()];
                    println!("Second slice: {second_slice}");
                    if let Some(first_match) = digit_regex.find(&line[line.len() - i..line.len()]) {
                        second = Some(slice_to_number(first_match.as_str()));
                        println!("Second: {}", second.unwrap());
                    }
                }
                if first.is_some() && second.is_some() {
                    break;
                }
            }

            first.unwrap_or(0) * 10 + second.unwrap_or(0)
        })
        .inspect(|n| println!("{n}"))
        .sum()
}

fn main() {
    let mut aoc = utils::AdventOfCode::new(2023, 1);
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

    #[test]
    fn part_one_test() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        let result = 142;

        assert_eq!(part_one(&input.to_string()), result);
    }

    #[test]
    fn part_two_test() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        let result = 281;

        assert_eq!(part_two(&input.to_string()), result);
    }
}
