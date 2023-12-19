use utils;

#[derive(Debug)]
enum Colors {
    Red(u32),
    Green(u32),
    Blue(u32),
}

struct Game {
    id: u32,
    red: Vec<u32>,
    green: Vec<u32>,
    blue: Vec<u32>,
}

impl Game {
    pub fn new(input: &str) -> Self {
        let first_split = input.split(':').collect::<Vec<&str>>();
        assert_eq!(first_split.len(), 2);
        let id = Self::get_id(first_split.first().unwrap());
        // let cubes = Self::add_cubes(first_split.last().unwrap());
        let cubes = Self::get_cubes(first_split.last().unwrap());

        Self {
            id,
            red: cubes.0,
            green: cubes.1,
            blue: cubes.2,
        }
    }

    pub fn validate_game(&self, red: u32, green: u32, blue: u32) -> bool {
        // self.red < red && self.green < green && self.blue < blue
        for cubes in self.red.iter() {
            if cubes > &red {
                return false;
            }
        }
        for cubes in self.green.iter() {
            if cubes > &green {
                return false;
            }
        }
        for cubes in self.blue.iter() {
            if cubes > &blue {
                return false;
            }
        }

        true
    }

    fn get_id(input: &str) -> u32 {
        let second_split = input.split(' ').collect::<Vec<&str>>();
        assert_eq!(second_split.len(), 2);
        second_split.last().unwrap().parse::<u32>().unwrap()
    }

    fn get_cubes(input: &str) -> (Vec<u32>, Vec<u32>, Vec<u32>) {
        input
            .split(';')
            .map(|hand| hand.split(',').collect::<Vec<&str>>())
            .flatten()
            .map(|cube| Self::parse_cubes(cube))
            .fold(
                (Vec::new(), Vec::new(), Vec::new()),
                |(mut red, mut green, mut blue), color| {
                    match color {
                        Colors::Red(count) => red.push(count),
                        Colors::Green(count) => green.push(count),
                        Colors::Blue(count) => blue.push(count),
                    }
                    (red, green, blue)
                },
            )
    }

    fn add_cubes(input: &str) -> (u32, u32, u32) {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        input
            .split(';')
            .map(|hand| hand.split(',').collect::<Vec<&str>>())
            .flatten()
            .map(|cube| Self::parse_cubes(cube))
            .for_each(|color| match color {
                Colors::Red(count) => red += count,
                Colors::Green(count) => green += count,
                Colors::Blue(count) => blue += count,
            });

        (red, green, blue)
    }

    fn parse_cubes(input: &str) -> Colors {
        let split = input.trim().split(' ').collect::<Vec<&str>>();
        assert_eq!(split.len(), 2);
        let count = split.first().unwrap().parse::<u32>().unwrap();
        match *split.last().unwrap() {
            "red" => Colors::Red(count),
            "green" => Colors::Green(count),
            "blue" => Colors::Blue(count),
            _ => panic!("Invalid color in the game"),
        }
    }

    fn get_power(&self) -> u32 {
        self.red.iter().max().unwrap()
            * self.green.iter().max().unwrap()
            * self.blue.iter().max().unwrap()
    }
}

fn part_one(input: &String) -> u32 {
    input
        .lines()
        .inspect(|line| println!("{line}"))
        .map(|line| Game::new(line))
        .filter_map(|game| {
            if game.validate_game(12, 13, 14) {
                Some(game.id)
            } else {
                None
            }
        })
        .inspect(|i| println!("{i}"))
        .sum()
}

fn part_two(input: &String) -> u32 {
    input
        .lines()
        .inspect(|line| println!("{line}"))
        .map(|line| {
            let game = Game::new(line);
            game.get_power()
        })
        .inspect(|i| println!("{i}"))
        .sum()
}

fn main() {
    let mut aoc = utils::AdventOfCode::new(2023, 2);
    let input = aoc.get_input().unwrap();

    let part_one = part_one(input);
    println!("Part one: {part_one}");

    let part_two = part_two(input);
    println!("Part two: {part_two}");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_test() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        let result = part_one(&input.to_string());

        assert_eq!(result, 8);
    }

    #[test]
    fn part2_test() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        let result = part_two(&input.to_string());

        assert_eq!(result, 2286);
    }
}
