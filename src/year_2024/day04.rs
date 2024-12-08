use std::collections::HashMap;

type Input = HashMap<(i32, i32), char>;

pub fn part1(input: &Input) -> usize {
    input
        .iter()
        .filter(|(_, char)| **char == 'X')
        .flat_map(|((x, y), _)| {
            (-1..=1)
                .flat_map(|dx| {
                    (-1..=1)
                        .map(move |dy| (dx, dy))
                        .filter(|(dx, dy)| (*dx, *dy) != (0, 0))
                })
                .filter(move |(dx, dy)| {
                    if let (Some(m), Some(a), Some(s)) = (
                        &&input.get(&(*x + *dx, *y + *dy)),
                        &&input.get(&(*x + 2 * *dx, *y + 2 * *dy)),
                        &input.get(&(*x + 3 * *dx, *y + 3 * *dy)),
                    ) {
                        **m == 'M' && **a == 'A' && **s == 'S'
                    } else {
                        false
                    }
                })
        })
        .count()
}

pub fn part2(_input: &Input) -> String {
    "not implemented".to_string()
}

pub fn parse(input: &str) -> HashMap<(i32, i32), char> {
    input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(move |(column, char)| ((row as i32, column as i32), char))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(EXAMPLE_INPUT)), 18);
    }
}
