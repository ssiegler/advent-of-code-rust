use std::collections::HashMap;

pub fn part1((left, right): &Input) -> u32 {
    let mut left = left.clone();
    left.sort_unstable();
    let mut right = right.clone();
    right.sort_unstable();

    left.iter().zip(right).map(|(a, b)| a.abs_diff(b)).sum()
}

pub fn part2((left, right): &Input) -> u32 {
    let mut counts = HashMap::new();
    right
        .iter()
        .for_each(|b| *counts.entry(b).or_insert(0) += 1);
    left.iter()
        .filter_map(|a| counts.get(a).map(|b| a * b))
        .sum()
}

type Input = (Vec<u32>, Vec<u32>);

pub fn parse(input: &str) -> Input {
    input
        .lines()
        .filter_map(|line| {
            if let [left, right] = line
                .split_whitespace()
                .map(|n| n.parse::<u32>().expect("numbers"))
                .take(2)
                .collect::<Vec<_>>()[..]
            {
                Some((left, right))
            } else {
                None
            }
        })
        .unzip()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn parses_example_lists() {
        assert_eq!(
            parse(EXAMPLE),
            (vec![3, 4, 2, 1, 3, 3], vec![4, 3, 5, 3, 9, 3])
        );
    }
    #[test]
    fn solves_part1() {
        let input = parse(EXAMPLE);
        assert_eq!(part1(&input), 11);
    }
    #[test]
    fn solves_part2() {
        let input = parse(EXAMPLE);
        assert_eq!(part2(&input), 31);
    }
}
