enum RPS {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

enum Game {
    Lost = 0,
    Draw = 3,
    Win = 6,
}

fn part_one(input: &String) {
    let result: i32 = input
        .split('\n')
        .filter_map(|line| {
            println!("{}", line);
            if line.len() == 0 {
                return None;
            }
            let instruction: Vec<u32> = line.chars().map(|x| x as u32).collect();
            let game_result = (instruction[2] as i32 - instruction[0] as i32 - 23).abs();
            println!("{}", game_result);
            let score = if game_result == 0 {
                3
            } else if game_result == 1 {
                6
            } else {
                0
            } + match instruction[2] {
                88 => 1,
                89 => 2,
                90 => 3,
                _ => panic!("Wrong tool"),
            };
            println!("{}", score);
            Some(score)
        })
        .sum();

    println!("Part one result: {}", result);
}

fn main() {
    let input = utils::get_input("https://adventofcode.com/2022/day/2/input".to_string()).unwrap();

    part_one(&input);
}

// Rock     A: 65 X: 88
// Paper    B: 66 Y: 89
// Scissors C: 67 Z: 90
/*
    88  89  90
65  23  24  25
66  22  23  24
67  21  22  23
*/
