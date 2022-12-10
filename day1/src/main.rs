use utils;

fn part_one(input: &String) {
    let max_calories: i32 = input
        .split("\n\n")
        .map(|elf| {
            elf.split('\n')
                .filter_map(|calories| calories.parse::<i32>().ok())
                .sum()
        })
        .max()
        .unwrap();

    println!("Part 1 answer: {}", max_calories);
}

fn part_two(input: &String) {
    let mut elfs_calories: Vec<i32> = input
        .split("\n\n")
        .map(|elf| {
            elf.split('\n')
                .filter_map(|calories| calories.parse::<i32>().ok())
                .sum()
        })
        .collect();
    elfs_calories.sort();
    elfs_calories.reverse();

    let top_three_sum: i32 = elfs_calories.iter().take(3).sum();

    println!("Part 2 answer: {}", top_three_sum);
}

fn main() {
    let input = utils::get_input("https://adventofcode.com/2022/day/1/input".to_string()).unwrap();

    part_one(&input);

    part_two(&input);
}
