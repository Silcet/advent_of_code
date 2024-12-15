use utils;

fn parse_input(input: &String) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|x| x.parse::<u32>().expect("Invalid input"))
                .collect()
        })
        .collect()
}

fn is_safe(report: &Vec<u32>) -> bool {
    (report.is_sorted_by(|a, b| a > b) || report.is_sorted_by(|a, b| a < b))
        && report
            .windows(2)
            .map(|w| (w[0] as i32 - w[1] as i32).abs())
            .filter(|&d| d > 3 || d == 0)
            .count()
            == 0
}

fn part_one(input: &String) -> u32 {
    let reports = parse_input(input);

    reports.iter().filter(|report| is_safe(report)).count() as u32
}

fn only_one_error(report: &Vec<u32>) -> bool {
    for i in 0..report.len() {
        let reduced_report: Vec<u32> = report
            .iter()
            .enumerate()
            .filter_map(|(index, x)| if i != index { Some(*x) } else { None })
            .collect();
        if is_safe(&reduced_report) {
            return true;
        }
    }
    false
}

fn part_two(input: &String) -> u32 {
    let reports = parse_input(input);

    reports.len() as u32
        - reports
            .iter()
            .filter(|report| !is_safe(report))
            .filter(|report| !only_one_error(report))
            .count() as u32
}

fn main() {
    let mut aoc = utils::AdventOfCode::new(2024, 2);
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
    const INPUT: &str = "7 6 4 2 1
    1 2 7 8 9
    9 7 6 2 1
    1 3 2 4 5
    8 6 4 4 1
    1 3 6 7 9
    9 5 4 3 2
    1 2 3 4 9";
    const RESULT_ONE: u32 = 2;
    const RESULT_TWO: u32 = 6;

    #[test]
    fn part_one_test() {
        assert_eq!(part_one(&INPUT.to_string()), RESULT_ONE);
    }

    #[test]
    fn part_two_test() {
        assert_eq!(part_two(&INPUT.to_string()), RESULT_TWO);
    }
}
