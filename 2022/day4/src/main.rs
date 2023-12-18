use std::cmp::Ordering;

fn part_one(input: &String) -> usize {
    input
        .split('\n')
        .filter_map(|line| {
            if line.is_empty() {
                None
            } else {
                Some(
                    line.split(',')
                        .map(|shift| {
                            shift
                                .split('-')
                                .map(|number| number.parse::<u32>().unwrap())
                                .collect()
                        })
                        .collect::<Vec<Vec<u32>>>(),
                )
            }
        })
        .filter(
            |shift| match (shift[0][0].cmp(&shift[1][0]), shift[0][1].cmp(&shift[1][1])) {
                (Ordering::Less, Ordering::Greater) => true,
                (Ordering::Equal, Ordering::Greater) => true,
                (Ordering::Less, Ordering::Equal) => true,
                (Ordering::Greater, Ordering::Less) => true,
                (Ordering::Equal, Ordering::Less) => true,
                (Ordering::Greater, Ordering::Equal) => true,
                (Ordering::Equal, Ordering::Equal) => true,
                _ => false,
            },
        )
        .count()
}

fn part_two(input: &String) -> usize {
    input
        .split('\n')
        .filter_map(|line| {
            if line.is_empty() {
                None
            } else {
                Some(
                    line.split(',')
                        .map(|shift| {
                            shift
                                .split('-')
                                .map(|number| number.parse::<u32>().unwrap())
                                .collect()
                        })
                        .collect::<Vec<Vec<u32>>>(),
                )
            }
        })
        .filter(|shift| {
            match (
                shift[0][0].cmp(&shift[1][0]),
                shift[0][1].cmp(&shift[1][0]),
                shift[0][1].cmp(&shift[1][1]),
            ) {
                (Ordering::Less, Ordering::Greater, _) => true,
                (Ordering::Less, Ordering::Equal, _) => true,
                (Ordering::Equal, Ordering::Equal, _) => true,
                (Ordering::Equal, Ordering::Greater, _) => true,
                (_, Ordering::Greater, Ordering::Less) => true,
                (_, Ordering::Greater, Ordering::Equal) => true,
                (_, Ordering::Equal, Ordering::Less) => true,
                (_, Ordering::Equal, Ordering::Equal) => true,
                _ => false,
            }
        })
        .count()
}

fn main() {
    let mut aoc = utils::AdventOfCode::new(2022, 4);
    let input = aoc.get_input().unwrap();

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
        let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"
            .to_string();
        assert_eq!(part_one(&input), 2);
    }

    #[test]
    fn part_two_example() {
        let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"
            .to_string();
        assert_eq!(part_two(&input), 4);
    }
}
