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

fn part_two(input: &String) -> u32 {
    let digit_map: HashMap<&str, &str> = HashMap::from([
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ]);

    input
        .lines()
        .inspect(|l| println!("{l}"))
        .map(|line| {
            let mut numeric_line = line.to_string();
            for (text, number) in digit_map.iter() {
                numeric_line = numeric_line.replace(text, number);
            }
            let mut numbers = numeric_line.chars().filter_map(|c| c.to_digit(10));
            println!("{numbers:?}");
            let first = numbers.next().unwrap_or(0);
            let second = numbers.last().unwrap_or(first);
            first * 10 + second
        })
        .inspect(|n| println!("{n}"))
        .sum()
}

fn main() {
    let input = utils::get_day_input(2023, 1).unwrap();

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
