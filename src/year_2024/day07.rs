use std::ops::Add;
use std::ops::Mul;
type Input = Vec<Vec<u64>>;

pub fn part1(input: &Input) -> u64 {
    input
        .iter()
        .filter(|numbers| possible_true(numbers, &[u64::add, u64::mul]))
        .map(|numbers| numbers[0])
        .sum()
}

pub fn part2(input: &Input) -> u64 {
    input
        .iter()
        .filter(|numbers| possible_true(numbers, &[u64::add, u64::mul, concatenate]))
        .map(|numbers| numbers[0])
        .sum()
}

pub fn parse(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|number| {
                    number
                        .strip_suffix(":")
                        .unwrap_or(number)
                        .parse::<u64>()
                        .expect("numbers")
                })
                .collect()
        })
        .collect()
}

fn combinations(numbers: &[u64], operators: &[fn(u64, u64) -> u64]) -> Vec<u64> {
    if numbers.is_empty() {
        vec![]
    } else {
        numbers[1..]
            .iter()
            .fold(vec![numbers[0]], |combinations, next| {
                combinations
                    .iter()
                    .flat_map(|combination| {
                        operators
                            .iter()
                            .map(|operator| operator(*combination, *next))
                    })
                    .collect()
            })
    }
}

fn possible_true(numbers: &[u64], operators: &[fn(u64, u64) -> u64]) -> bool {
    combinations(&numbers[1..], operators)
        .iter()
        .any(|result| *result == numbers[0])
}

fn concatenate(left: u64, right: u64) -> u64 {
    format!("{}{}", left, right).parse().expect("valid result")
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "\
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn possible_true_examples() {
        let results = parse(EXAMPLE_INPUT)
            .iter()
            .map(|equation| possible_true(equation, &[u64::add, u64::mul]))
            .collect::<Vec<_>>();
        assert_eq!(
            results,
            vec![true, true, false, false, false, false, false, false, true]
        );
    }

    #[test]
    fn solves_example_part1() {
        assert_eq!(part1(&parse(EXAMPLE_INPUT)), 3749);
    }

    #[test]
    fn solves_example_part2() {
        assert_eq!(part2(&parse(EXAMPLE_INPUT)), 11387);
    }
}
