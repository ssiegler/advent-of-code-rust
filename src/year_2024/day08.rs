use itertools::iterate;
use std::collections::{HashMap, HashSet};

#[derive(PartialEq, Debug)]
pub struct Input {
    antennas: HashMap<char, Vec<(i32, i32)>>,
    width: i32,
    height: i32,
}

pub fn part1(input: &Input) -> usize {
    input
        .antennas
        .values()
        .flat_map(|antennas| antinodes(antennas))
        .filter(|(row, column)| {
            (0..input.height).contains(row) && (0..input.width).contains(column)
        })
        .collect::<HashSet<_>>()
        .len()
}

pub fn part2(input: &Input) -> usize {
    input
        .antennas
        .values()
        .flat_map(|antennas| all_antinodes(antennas, input.height, input.width))
        .collect::<HashSet<_>>()
        .len()
}

pub fn parse(input: &str) -> Input {
    let mut antennas = HashMap::new();
    let mut width = 0;
    let mut height = 0;
    input.lines().enumerate().for_each(|(row, line)| {
        width = line.len() as i32;
        height += 1;
        line.chars()
            .enumerate()
            .filter(|(_, char)| char.is_alphanumeric())
            .for_each(|(column, char)| {
                antennas
                    .entry(char)
                    .or_insert(Vec::new())
                    .push((row as i32, column as i32));
            });
    });
    Input {
        antennas,
        width,
        height,
    }
}

fn antinodes(antennas: &[(i32, i32)]) -> impl Iterator<Item = (i32, i32)> + use<'_> {
    antennas.iter().flat_map(|antenna1 @ (row1, column1)| {
        antennas
            .iter()
            .filter(move |antenna2| antenna1 != *antenna2)
            .flat_map(move |(row2, column2)| {
                let (drow, dcolumn) = (row2 - row1, column2 - column1);
                std::iter::once((row2 + drow, column2 + dcolumn))
            })
    })
}

fn all_antinodes(
    antennas: &[(i32, i32)],
    height: i32,
    width: i32,
) -> impl Iterator<Item = (i32, i32)> + use<'_> {
    antennas.iter().flat_map(move |antenna1 @ (row1, column1)| {
        antennas
            .iter()
            .filter(move |antenna2| antenna1 != *antenna2)
            .flat_map(move |(row2, column2)| {
                let (drow, dcolumn) = (row2 - row1, column2 - column1);
                iterate((*row2, *column2), move |(row, column)| {
                    (*row + drow, *column + dcolumn)
                })
                .take_while(move |(row, column)| {
                    (0..height).contains(row) && (0..width).contains(column)
                })
            })
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "\
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
";

    #[test]
    fn parses_example() {
        assert_eq!(
            parse(EXAMPLE_INPUT),
            Input {
                antennas: [
                    ('A', vec![(5, 6), (8, 8), (9, 9)]),
                    ('0', vec![(1, 8), (2, 5), (3, 7), (4, 4)]),
                ]
                .into_iter()
                .collect::<HashMap<_, _>>(),
                width: 12,
                height: 12,
            }
        );
    }

    #[test]
    fn creates_antinodes() {
        assert_eq!(
            antinodes(&[(3, 4), (5, 5)]).collect::<Vec<_>>(),
            vec![(7, 6), (1, 3)]
        );
    }

    #[test]
    fn solves_part1() {
        assert_eq!(part1(&parse(EXAMPLE_INPUT)), 14);
    }

    #[test]
    fn solves_part2() {
        assert_eq!(part2(&parse(EXAMPLE_INPUT)), 34);
    }
}
