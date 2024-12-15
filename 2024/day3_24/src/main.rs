use regex::Regex;
use utils;

fn part_one(input: &String) -> u32 {
    let re = Regex::new(r"mul\((?P<num1>\d{1,3}),(?P<num2>\d{1,3})\)").unwrap();
    re.captures_iter(input)
        .map(|capture| {
            let (_, [l, r]) = capture.extract();
            l.parse::<u32>().expect("Invalid input") * r.parse::<u32>().expect("Invalid input")
        })
        .sum()
}

fn part_two(input: &String) -> u32 {
    let re = Regex::new(
        r"(mul\((?P<num1>\d{1,3}),(?P<num2>\d{1,3})\))|(?P<do>do\(\))|(?P<dont>don't\(\))",
    )
    .unwrap();
    let mut active = true;
    re.captures_iter(input)
        .filter_map(|capture| {
            if capture.name("do").is_some() {
                active = true;
                return None;
            } else if capture.name("dont").is_some() {
                active = false;
                return None;
            }

            if active {
                match (capture.name("num1"), capture.name("num2")) {
                    (Some(l), Some(r)) => {
                        return Some(
                            l.as_str().parse::<u32>().expect("Invalid input")
                                * r.as_str().parse::<u32>().expect("Invalid input"),
                        )
                    }
                    _ => return None,
                }
            };

            None
        })
        .sum()
}

fn main() {
    let mut aoc = utils::AdventOfCode::new(2024, 3);
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
    const INPUT: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const INPUT2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    const RESULT_ONE: u32 = 161;
    const RESULT_TWO: u32 = 48;

    #[test]
    fn part_one_test() {
        assert_eq!(part_one(&INPUT.to_string()), RESULT_ONE);
    }

    #[test]
    fn part_two_test() {
        assert_eq!(part_two(&INPUT2.to_string()), RESULT_TWO);
    }
}
