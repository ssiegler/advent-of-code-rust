use crate::parser;
use anyhow::Result;
use itertools::Itertools;
use nom::character::complete::{multispace1, newline};
use nom::combinator::complete;
use nom::multi::separated_list0;
use nom::sequence::separated_pair;
use parser::integer;

pub(super) fn solution(input: &[u8]) -> Result<(String, String)> {
    let (left, right) = {
        let (mut left, mut right) = parse(input)?;
        left.sort();
        right.sort();
        (left, right)
    };
    let total_distance: u32 = left.iter().zip(&right).map(|(a, b)| a.abs_diff(*b)).sum();
    let counts = right.iter().counts();
    let similarity_score: usize = left
        .iter()
        .map(|number| *number as usize * counts.get(number).cloned().unwrap_or(0))
        .sum();
    Ok((total_distance.to_string(), similarity_score.to_string()))
}

fn parse(input: &[u8]) -> Result<(Vec<u32>, Vec<u32>)> {
    let paired = complete(separated_list0(
        newline,
        separated_pair(integer, multispace1, integer),
    ))(input)
    .map(|(_, values)| values)
    .map_err(|err| err.to_owned())?;
    Ok(paired.iter().cloned().unzip())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &[u8; 35] = b"3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn parses_example_lists() -> Result<()> {
        assert_eq!(
            parse(EXAMPLE)?,
            (vec![3, 4, 2, 1, 3, 3], vec![4, 3, 5, 3, 9, 3])
        );
        Ok(())
    }
    #[test]
    fn solve_example() -> Result<()> {
        assert_eq!(solution(EXAMPLE)?, ("11".to_string(), "31".to_string()));
        Ok(())
    }
}
