use itertools::{iterate, Itertools};
use std::collections::HashMap;

type Input = HashMap<u64, usize>;

pub fn part1(input: &Input) -> usize {
    iterate(input.clone(), blink)
        .nth(25)
        .expect("stones after 25 blinks")
        .values()
        .sum()
}

pub fn part2(input: &Input) -> usize {
    iterate(input.clone(), blink)
        .nth(75)
        .expect("stones after 75 blinks")
        .values()
        .sum()
}

pub fn parse(input: &str) -> Input {
    input
        .split_whitespace()
        .map(|stone| stone.parse::<u64>().expect("engraved number"))
        .counts()
}

fn blink(input: &Input) -> Input {
    let mut stones = HashMap::new();
    for (stone, times) in input {
        for stone in stone_rules(stone) {
            *stones.entry(stone).or_insert(0) += times
        }
    }
    stones
}

fn stone_rules(stone: &u64) -> Vec<u64> {
    match *stone {
        0 => vec![1],
        1..10 => vec![stone.checked_mul(2024).expect("u64 to be big enough")],
        10..100 => vec![*stone / 10, *stone % 10],
        100..1000 => vec![stone.checked_mul(2024).expect("u64 to be big enough")],
        1000..10000 => vec![*stone / 100, *stone % 100],
        10000..100000 => vec![stone.checked_mul(2024).expect("u64 to be big enough")],
        100000..1000000 => vec![*stone / 1000, *stone % 1000],
        1000000..10000000 => vec![stone.checked_mul(2024).expect("u64 to be big enough")],
        10000000..100000000 => vec![*stone / 10000, *stone % 10000],
        100000000..1000000000 => vec![stone.checked_mul(2024).expect("u64 to be big enough")],
        1000000000..10000000000 => vec![*stone / 100000, *stone % 100000],
        10000000000..100000000000 => {
            vec![stone.checked_mul(2024).expect("u64 to be big enough")]
        }
        100000000000..1000000000000 => vec![*stone / 1000000, *stone % 1000000],
        1000000000000..10000000000000 => {
            vec![stone.checked_mul(2024).expect("u64 to be big enough")]
        }
        10000000000000..100000000000000 => vec![*stone / 10000000, *stone % 10000000],
        100000000000000..1000000000000000 => {
            vec![stone.checked_mul(2024).expect("u64 to be big enough")]
        }
        1000000000000000..10000000000000000 => vec![*stone / 100000000, *stone % 100000000],
        10000000000000000..100000000000000000 => {
            vec![stone.checked_mul(2024).expect("u64 to be big enough")]
        }
        100000000000000000..1000000000000000000 => {
            vec![*stone / 1000000000, *stone % 1000000000]
        }
        1000000000000000000..10000000000000000000 => {
            vec![stone.checked_mul(2024).expect("u64 to be big enough")]
        }
        10000000000000000000..=u64::MAX => vec![*stone / 10000000000, *stone % 10000000000],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(&[0, 1, 10, 99, 999],
           &[1, 2024, 1, 0, 9, 9, 2021976])]
    #[case(&[125, 17],
           &[253000, 1, 7])]
    #[case(&[253000, 1, 7],
           &[253, 0, 2024, 14168])]
    #[case(&[253, 0, 2024, 14168],
           &[512072, 1, 20, 24, 28676032])]
    #[case(&[512072, 1, 20, 24, 28676032],
           &[512,72,2024,2,0,2,4,2867,6032])]
    #[case(&[512,72,2024,2,0,2,4,2867,6032],
           &[1036288, 7, 2, 20, 24, 4048, 1, 4048, 8096, 28, 67, 60, 32])]
    #[case(&[1036288, 7, 2, 20, 24, 4048, 1, 4048, 8096, 28, 67, 60, 32],
           &[2097446912,14168,4048,2,0,2,4,40,48,2024,40,48,80,96,2,8,6,7,6,0,3,2])]
    fn blinks(#[case] input: &[u64], #[case] output: &[u64]) {
        assert_eq!(
            blink(&input.iter().cloned().counts()),
            output.iter().cloned().counts()
        );
    }

    #[test]
    fn solves_part1() {
        assert_eq!(55312, part1(&parse("125 17")));
    }
}
