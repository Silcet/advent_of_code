use std::{collections::HashMap, ops::Range};
use utils;

type Pos = (i32, i32);

const SQUARE: &[Pos; 8] = &[
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

const DIAGONAL_ONE: &[Pos; 2] = &[(-1, -1), (1, 1)];
const DIAGONAL_TWO: &[Pos; 2] = &[(1, -1), (-1, 1)];

fn parse_input(input: &String) -> HashMap<Pos, char> {
    input
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.trim()
                .chars()
                .enumerate()
                .map(move |(col, c)| ((row as i32, col as i32), c))
        })
        .flatten()
        .collect()
}

fn get_input_limits(input: &String) -> (Range<i32>, Range<i32>) {
    let max_row = input.lines().count() as i32;
    let max_col = input.lines().next().unwrap().chars().count() as i32;

    (0..max_row, 0..max_col)
}

fn get_valid_neighbours(
    puzzle: &HashMap<Pos, char>,
    pos: &Pos,
    letters: &[char],
    input: &String,
    matrix: &[Pos],
) -> Vec<Pos> {
    let (row_limits, col_limits) = get_input_limits(input);
    matrix
        .iter()
        .filter_map(|(row, col)| {
            let temp_row = pos.0 - row;
            let temp_col = pos.1 - col;
            if row_limits.contains(&temp_row) && col_limits.contains(&temp_col) {
                Some((temp_row, temp_col))
            } else {
                None
            }
        })
        .filter(|p| letters.contains(puzzle.get(p).unwrap()))
        .collect()
}

fn part_one(input: &String) -> u32 {
    let puzzle = parse_input(input);

    let xs: Vec<Pos> = puzzle
        .iter()
        .filter_map(|(p, &c)| if c == 'X' { Some(*p) } else { None })
        .collect();
    xs.iter()
        .map(|x| {
            get_valid_neighbours(&puzzle, x, &['M'], input, SQUARE)
                .iter()
                .map(|m| (*m, (m.0 - x.0, m.1 - x.1)))
                .collect::<Vec<(Pos, Pos)>>()
        })
        .flatten()
        .map(|(m, d)| {
            get_valid_neighbours(&puzzle, &m, &['A'], input, SQUARE)
                .iter()
                .filter(|&a| a == &(m.0 + d.0, m.1 + d.1))
                .map(|a| (*a, (a.0 - m.0, a.1 - m.1)))
                .collect::<Vec<(Pos, Pos)>>()
        })
        .flatten()
        .map(|(a, d)| {
            get_valid_neighbours(&puzzle, &a, &['S'], input, SQUARE)
                .iter()
                .copied()
                .filter(|s| *s == (a.0 + d.0, a.1 + d.1))
                .collect::<Vec<Pos>>()
        })
        .flatten()
        .count() as u32
}

fn part_two(input: &String) -> u32 {
    let puzzle = parse_input(input);

    puzzle
        .iter()
        .filter_map(|(p, &c)| if c == 'A' { Some(*p) } else { None })
        .filter_map(|p| {
            let diag_one_m = get_valid_neighbours(&puzzle, &p, &['M'], &input, DIAGONAL_ONE);
            let diag_one_s = get_valid_neighbours(&puzzle, &p, &['S'], &input, DIAGONAL_ONE);
            let diag_two_m = get_valid_neighbours(&puzzle, &p, &['M'], &input, DIAGONAL_TWO);
            let diag_two_s = get_valid_neighbours(&puzzle, &p, &['S'], &input, DIAGONAL_TWO);
            if diag_one_m.len() == 1
                && diag_one_s.len() == 1
                && diag_two_m.len() == 1
                && diag_two_s.len() == 1
            {
                Some(())
            } else {
                None
            }
        })
        .count() as u32
}

fn main() {
    let mut aoc = utils::AdventOfCode::new(2024, 4);
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
    const INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
    const RESULT_ONE: u32 = 18;
    const RESULT_TWO: u32 = 9;

    #[test]
    fn get_neighbours_corner_test() {
        const RESULT: &[Pos; 2] = &[(8, 9), (8, 8)];
        let puzzle = parse_input(&INPUT.to_string());
        let neighbours = get_valid_neighbours(&puzzle, &(9, 9), &['M'], &INPUT.to_string(), SQUARE);
        assert_eq!(neighbours.len(), RESULT.len());
        for letter in neighbours {
            assert!(RESULT.contains(&letter));
        }
    }

    #[test]
    fn get_neighbours_side_test() {
        const RESULT: &[Pos; 5] = &[(3, 8), (2, 8), (2, 9), (4, 8), (4, 9)];
        let puzzle = parse_input(&INPUT.to_string());
        let neighbours = get_valid_neighbours(&puzzle, &(3, 9), &['M'], &INPUT.to_string(), SQUARE);
        assert_eq!(neighbours.len(), RESULT.len());
        for letter in neighbours {
            assert!(RESULT.contains(&letter));
        }
    }

    #[test]
    fn part_one_test() {
        assert_eq!(part_one(&INPUT.to_string()), RESULT_ONE);
    }

    #[test]
    fn part_two_test() {
        assert_eq!(part_two(&INPUT.to_string()), RESULT_TWO);
    }
}
