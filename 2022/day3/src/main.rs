fn part_one(input: &String) -> u32 {
    input.split('\n').filter(|line| !line.is_empty()).map(|line| {
        line.split_at(line.len()/2)
    }).inspect(|split| println!("{}\n{}", split.0, split.1)).map(|rucksack| {
        rucksack.0.chars().find(|item|
            rucksack.1.contains(*item)
        ).map(|item| {
            if item.is_ascii_lowercase() {
                item as u32 - 96
            } else {
                item as u32 - 38
            }
        })
    }).map(|item| item.unwrap()).inspect(|item| println!("{}", char::from_u32(*item).unwrap())).sum()
}

fn part_two(input: &String) -> u32 {
    let rugsacks: Vec<&str> = input.split('\n').filter(|line| !line.is_empty()).collect();
    rugsacks.chunks(3).map(|group| {
        group[0].chars().find(|item| group[1].contains(*item) && group[2].contains(*item)).map(|item| {
            if item.is_ascii_lowercase() {
                item as u32 - 96
            } else {
                item as u32 - 38
            }
        })
    }).map(|item| item.unwrap()).sum()
}

fn main() {
    let input = utils::get_input("https://adventofcode.com/2022/day/3/input".to_string()).unwrap();

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
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw".to_string();
        assert_eq!(part_one(&input), 157u32);
    }

    #[test]
    fn part_two_example() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw".to_string();
        assert_eq!(part_two(&input), 70u32);
    }
}
