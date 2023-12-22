use utils;

#[derive(Debug)]
struct PartNumber {
    row: usize,
    start: usize,
    end: usize,
    number: usize,
}

#[derive(Debug)]
struct Symbol {
    row: usize,
    index: usize,
    symbol: char,
}

#[derive(Debug)]
struct Schematic {
    part_numbers: Vec<PartNumber>,
    symbols: Vec<Symbol>,
    max_row: usize,
    max_col: usize,
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
                            symbols.push(Symbol {
                                row,
                                index,
                                symbol: c,
                            });
                        }
                        index += 1;
                    });
            });

        // Get part numbers
        input
            .lines()
            .enumerate()
            // .inspect(|s| println!("{s:?}"))
            .for_each(|(row, line)| {
                let mut index = 0;
                line.split(|c: char| !c.is_digit(10))
                    // .inspect(|s| println!("{s:?}"))
                    .for_each(|entry| {
                        if !entry.is_empty() {
                            match entry.parse::<usize>() {
                                Ok(number) => part_numbers.push(PartNumber {
                                    row,
                                    start: index,
                                    end: index + entry.len() - 1,
                                    number,
                                }),
                                Err(_) => (),
                            }
                        }
                        index += if entry.is_empty() { 1 } else { entry.len() + 1 };
                    });
            });

        // println!("{part_numbers:?}");
        // println!("{symbols:?}");

        Self {
            part_numbers,
            symbols,
            max_row: input.lines().count(),
            max_col: input.lines().next().unwrap().len(),
        }
    }

    fn get_valid_part_numbers(&self) -> Vec<&PartNumber> {
        self.part_numbers
            .iter()
            .filter(|pn| self.validate_part_number(pn))
            .collect()
    }

    fn validate_part_number(&self, pn: &PartNumber) -> bool {
        let min_row = pn.row.checked_sub(1).unwrap_or(0);
        let max_row = if pn.row + 1 <= self.max_row {
            pn.row + 1
        } else {
            pn.row
        };
        let min_col = pn.start.checked_sub(1).unwrap_or(0);
        let max_col = if pn.end + 1 <= self.max_col {
            pn.end + 1
        } else {
            pn.end
        };

        for symbol in self.symbols.iter() {
            // println!(
            //     "{min_row},{max_row};{min_col},{max_col} - {0},{1}",
            //     symbol.row, symbol.index
            // );
            if (min_row..=max_row).contains(&symbol.row)
                && (min_col..=max_col).contains(&symbol.index)
            {
                return true;
            }
        }
        false
    }

    pub fn get_gears(&self) -> Vec<usize> {
        self.symbols
            .iter()
            .filter_map(|s| {
                if s.symbol == '*' {
                    let min_row = s.row.checked_sub(1).unwrap_or(0);
                    let max_row = if s.row + 1 <= self.max_row {
                        s.row + 1
                    } else {
                        s.row
                    };
                    let min_col = s.index.checked_sub(1).unwrap_or(0);
                    let max_col = if s.index + 1 <= self.max_col {
                        s.index + 1
                    } else {
                        s.index
                    };

                    let row_range = min_row..=max_row;
                    let col_range = min_col..=max_col;

                    let gears: Vec<usize> = self
                        .part_numbers
                        .iter()
                        .filter_map(|pn| {
                            // println!(
                            //     "{row_range:?}:{col_range:?}-{0},{1},{2}",
                            //     pn.row, pn.start, pn.end
                            // );
                            if row_range.contains(&pn.row) {
                                for i in pn.start..=pn.end {
                                    if col_range.contains(&i) {
                                        return Some(pn.number);
                                    }
                                }
                            }
                            None
                        })
                        .collect();

                    if gears.len() == 2 {
                        Some(gears.get(0).unwrap() * gears.get(1).unwrap())
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect()
    }
}

fn part_one(input: &String) -> u32 {
    let schematic = Schematic::new(input);
    schematic
        .get_valid_part_numbers()
        .iter()
        .map(|pn| pn.number)
        .sum::<usize>() as u32
}

fn part_two(input: &String) -> u32 {
    let schematic = Schematic::new(input);
    schematic.get_gears().iter().sum::<usize>() as u32
}

fn main() {
    let mut aoc = utils::AdventOfCode::new(2023, 3);
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
