use utils;

fn part_one(input: &String) -> u32 {
    unimplemented!()
}

fn part_two(input: &String) -> u32 {
    unimplemented!()
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
    const INPUT: &str = "";
    const RESULT_ONE: u32 = 0;
    const RESULT_TWO: u32 = 0;

    #[test]
    fn part_one_test() {
        assert_eq!(part_one(&INPUT.to_string()), RESULT_ONE);
    }

    #[test]
    fn part_two_test() {
        assert_eq!(part_two(&INPUT.to_string()), RESULT_TWO);
    }
}
