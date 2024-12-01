use crate::parser::integer;
use nom::character::complete::{char, newline};
use nom::combinator::{all_consuming, map, opt};
use nom::multi::separated_list0;
use nom::sequence::{terminated, tuple};

pub(super) fn solution(input: &[u8]) -> anyhow::Result<(String, String)> {
    let presents = parse_presents(input)?;
    Ok((
        presents.iter().map(required_paper).sum::<u32>().to_string(),
        presents.iter().map(required_rope).sum::<u32>().to_string(),
    ))
}

type Present = (u32, u32, u32);

fn required_paper((l, w, h): &Present) -> u32 {
    let sides = [l * w, l * h, w * h];
    sides.iter().min().unwrap() + sides.iter().sum::<u32>() * 2
}

fn required_rope((l, w, h): &Present) -> u32 {
    let mut edges = [*l, *w, *h];
    edges.sort();
    let edges = edges;
    2 * edges[0..=1].iter().sum::<u32>() + edges.iter().product::<u32>()
}

fn parse_presents(input: &[u8]) -> anyhow::Result<Vec<Present>> {
    let (_, presents) = all_consuming(terminated(
        separated_list0(
            newline,
            map(
                tuple((integer, char('x'), integer, char('x'), integer)),
                |(l, _, w, _, h)| (l, w, h),
            ),
        ),
        opt(newline),
    ))(input)
    .map_err(|e| e.to_owned())?;
    Ok(presents)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn parses_presents() -> anyhow::Result<()> {
        assert_eq!(
            vec![(2, 3, 4), (1, 1, 10)],
            parse_presents(b"2x3x4\n1x1x10")?
        );
        Ok(())
    }

    #[rstest]
    #[case((2,3,4), 58)]
    #[case((1,1,10), 43)]
    fn solves_part1_examples(#[case] present: Present, #[case] wrap_area: u32) {
        assert_eq!(required_paper(&present), wrap_area);
    }

    #[rstest]
    #[case((2,3,4), 34)]
    #[case((1,1,10), 14)]
    fn solves_part2_examples(#[case] present: Present, #[case] wrap_area: u32) {
        assert_eq!(required_rope(&present), wrap_area);
    }
}
