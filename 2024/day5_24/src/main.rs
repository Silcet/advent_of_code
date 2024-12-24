use utils;

fn parse_input(input: &String) -> (Vec<(u32, u32)>, Vec<Vec<u32>>) {
    let mut ordering_rules = Vec::<(u32, u32)>::new();
    let mut updates = Vec::new();

    for line in input.lines() {
        if line.contains('|') {
            let split_ordering: Vec<_> = line
                .trim()
                .split('|')
                .map(|x| x.parse::<u32>().expect("Invalid ordering rule input"))
                .collect();
            ordering_rules.push((
                *split_ordering.first().expect("Malformed ordering rule"),
                *split_ordering.last().expect("Malformed ordering rule"),
            ));
        } else if line.contains(',') {
            let split_updates = line
                .trim()
                .split(',')
                .map(|x| x.parse::<u32>().expect("Invalid ordering rule input"))
                .collect();
            updates.push(split_updates);
        }
    }

    (ordering_rules, updates)
}

fn part_one(input: &String) -> u32 {
    let (ordering_rules, updates) = parse_input(input);

    updates
        .iter()
        .filter(|update| {
            !ordering_rules.iter().any(|(k, v)| {
                let left_pos = update.iter().position(|x| x == k);
                let right_pos = update.iter().position(|x| x == v);
                match (left_pos, right_pos) {
                    (Some(x), Some(y)) => {
                        if x > y {
                            true
                        } else {
                            false
                        }
                    }
                    _ => false,
                }
            })
        })
        .map(|update| update[update.len() / 2])
        .sum()
}

fn order(ordering_rules: &Vec<(u32, u32)>, update: &mut Vec<u32>) -> bool {
    let change = ordering_rules.iter().find_map(|(k, v)| {
        let left_value = update.iter().position(|x| x == k);
        let right_value = update.iter().position(|x| x == v);

        match (left_value, right_value) {
            (Some(x), Some(y)) => {
                if x > y {
                    Some((x, y))
                } else {
                    None
                }
            }
            _ => None,
        }
    });

    if let Some((k, v)) = change {
        update.swap(k, v);
        return true;
    }

    false
}

fn part_two(input: &String) -> u32 {
    let (ordering_rules, mut updates) = parse_input(input);

    updates
        .iter_mut()
        .filter(|update| {
            ordering_rules.iter().any(|(k, v)| {
                let left_pos = update.iter().position(|x| x == k);
                let right_pos = update.iter().position(|x| x == v);
                match (left_pos, right_pos) {
                    (Some(x), Some(y)) => {
                        if x > y {
                            true
                        } else {
                            false
                        }
                    }
                    _ => false,
                }
            })
        })
        // .inspect(|x| println!("{x:?}"))
        .map(|mut update| {
            let mut change = true;
            let mut iter_count = 0;
            while change {
                change = order(&ordering_rules, &mut update);
                iter_count += 1;
            }
            update
        })
        // .inspect(|x| println!("{x:?}"))
        .map(|update| update[update.len() / 2])
        .sum()
}

fn main() {
    let mut aoc = utils::AdventOfCode::new(2024, 5);
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
    const INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
    const RESULT_ONE: u32 = 143;
    const RESULT_TWO: u32 = 123;

    #[test]
    fn part_one_test() {
        assert_eq!(part_one(&INPUT.to_string()), RESULT_ONE);
    }

    #[test]
    fn part_two_test() {
        assert_eq!(part_two(&INPUT.to_string()), RESULT_TWO);
    }
}
