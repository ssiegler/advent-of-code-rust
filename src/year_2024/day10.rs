use itertools::Itertools;
use std::collections::HashMap;

type Input = HashMap<(isize, isize), u8>;

pub fn part1(input: &Input) -> usize {
    trail_heads(input)
        .map(|trail_head| score(input, trail_head))
        .sum()
}

pub fn part2(input: &Input) -> usize {
    trail_heads(input)
        .map(|trail_head| rate(input, trail_head))
        .sum()
}

pub fn parse(input: &str) -> Input {
    input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.bytes()
                .enumerate()
                .map(move |(column, byte)| ((row as isize, column as isize), byte - b'0'))
        })
        .collect()
}

fn trail_heads(input: &Input) -> impl Iterator<Item = (isize, isize)> + use<'_> {
    input
        .iter()
        .filter(|(_, height)| **height == 0)
        .map(|(position, _)| position)
        .cloned()
}

fn score(input: &Input, position: (isize, isize)) -> usize {
    let mut positions = vec![position];
    for height in 1..10 {
        positions = positions
            .iter()
            .flat_map(|position| neighbors(*position))
            .collect();
        positions.sort_unstable();
        positions.dedup();
        positions.retain_mut(|position| input.get(position).map(|h| *h == height).unwrap_or(false));
    }
    positions.len()
}

fn rate(input: &Input, position: (isize, isize)) -> usize {
    let mut paths = vec![vec![position]];
    for height in 1..10 {
        paths = paths
            .iter()
            .flat_map(|path| {
                path.last()
                    .iter()
                    .flat_map(|position| neighbors(**position))
                    .filter(|position| input.get(position).map(|h| *h == height).unwrap_or(false))
                    .map(|position| {
                        let mut new_path = path.clone();
                        new_path.push(position);
                        new_path
                    })
                    .collect_vec()
            })
            .collect_vec();
    }
    paths.len()
}

fn neighbors((row, column): (isize, isize)) -> impl Iterator<Item = (isize, isize)> {
    [
        (row - 1, column),
        (row + 1, column),
        (row, column - 1),
        (row, column + 1),
    ]
    .into_iter()
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;

    const EXAMPLE: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    #[test]
    fn finds_trailheads() {
        assert_eq!(9, trail_heads(&(parse(EXAMPLE))).count());
    }

    #[test]
    fn scores_trailheads() {
        let input = parse(EXAMPLE);
        assert_eq!(
            vec![5, 6, 5, 3, 1, 3, 5, 3, 5],
            trail_heads(&(input))
                .sorted()
                .map(|position| { score(&input, position) })
                .collect_vec()
        );
    }

    #[test]
    fn solves_part1() {
        assert_eq!(part1(&parse(EXAMPLE)), 36);
    }

    #[test]
    fn rates_trailheads() {
        let input = parse(EXAMPLE);
        assert_eq!(
            vec![20, 24, 10, 4, 1, 4, 5, 8, 5],
            trail_heads(&(input))
                .sorted()
                .map(|position| { rate(&input, position) })
                .collect_vec()
        );
    }

    #[test]
    fn solves_part2() {
        assert_eq!(part2(&parse(EXAMPLE)), 81);
    }
}
