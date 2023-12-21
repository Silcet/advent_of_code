use utils;

#[derive(Debug)]
struct Schematic {
    part_numbers: Vec<usize>,
}

impl Schematic {
    pub fn new(input: &String) -> Self {
        let mut part_numbers = Vec::new();
        let mut symbols = Vec::new();

        // Get symbols
        input
            .lines()
            .enumerate()
            .inspect(|s| println!("{s:?}"))
            .for_each(|(row, line)| {
                let mut index = 0;
                line.chars()
                    // .inspect(|s| println!("{s:?}"))
                    .for_each(|c| {
                        if c != '.' && !c.is_digit(10) {
                            symbols.push((row, index))
                        }
                        index += 1;
                    });
            });

        let max_row = input.lines().count();
        let max_col = input.lines().next().unwrap().len();

        // Get part numbers
        input
            .lines()
            .enumerate()
            // .inspect(|s| println!("{s:?}"))
            .for_each(|(row, line)| {
                let mut index = 0;
                line.split(|c: char| !c.is_digit(10))
                    .inspect(|s| println!("{s:?}"))
                    .for_each(|entry| {
                        if !entry.is_empty()
                            && Self::validate_part_number(
                                row,
                                index,
                                entry.len(),
                                max_row,
                                max_col,
                                &symbols,
                            )
                        {
                            match entry.parse::<usize>() {
                                Ok(number) => part_numbers.push(number),
                                Err(_) => (),
                            }
                        }
                        index += if entry.is_empty() { 1 } else { entry.len() + 1 };
                    });
            });

        println!("{part_numbers:?}");
        println!("{symbols:?}");

        Self { part_numbers }
    }

    fn validate_part_number(
        row: usize,
        start: usize,
        len: usize,
        max_row: usize,
        max_col: usize,
        symbols: &Vec<(usize, usize)>,
    ) -> bool {
        let min_row = row.checked_sub(1).unwrap_or(0);
        let max_row = if row + 1 <= max_row { row + 1 } else { row };
        let min_col = start.checked_sub(1).unwrap_or(0);
        let end = start + len - 1;
        let max_col = if end + 1 <= max_col { end + 1 } else { end };

        for symbol in symbols.iter() {
            println!(
                "{min_row},{max_row};{min_col},{max_col} - {0},{1}",
                symbol.0, symbol.1
            );
            if (min_row..=max_row).contains(&symbol.0) && (min_col..=max_col).contains(&symbol.1) {
                return true;
            }
        }
        false
    }
}

fn part_one(input: &String) -> u32 {
    let schematic = Schematic::new(input);
    schematic.part_numbers.iter().sum::<usize>() as u32
}

fn part_two(input: &String) -> u32 {
    let schematic = Schematic::new(input);
    schematic.part_numbers.iter().sum::<usize>() as u32
}

fn main() {
    let mut aoc = utils::AdventOfCode::new(2023, 3);
    let input = aoc.get_input().unwrap();

    let part_one = part_one(input);
    println!("Part one: {part_one}");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        let result = 4361;

        assert_eq!(part_one(&input.to_string()), result);
    }

    #[test]
    fn test_part_two() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        let result = 467835;

        assert_eq!(part_two(&input.to_string()), result);
    }
}
