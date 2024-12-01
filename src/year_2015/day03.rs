use std::collections::HashSet;
use std::iter::once;

pub(super) fn solution(input: &[u8]) -> anyhow::Result<(String, String)> {
    Ok((
        scan_positions(input.iter())
            .collect::<HashSet<_>>()
            .len()
            .to_string(),
        scan_positions(input.iter().step_by(2))
            .chain(scan_positions(input[1..].iter().step_by(2)))
            .collect::<HashSet<_>>()
            .len()
            .to_string(),
    ))
}

type Position = (isize, isize);

fn scan_positions<'a>(
    input: impl Iterator<Item = &'a u8> + 'a,
) -> impl Iterator<Item = Position> + 'a {
    once((0, 0)).chain(input.scan((0, 0), |(x, y): &mut Position, b| {
        match *b {
            b'>' => {
                *x += 1;
            }
            b'^' => {
                *y += 1;
            }
            b'<' => {
                *x -= 1;
            }
            b'v' => {
                *y -= 1;
            }
            _ => {}
        }
        Some((*x, *y))
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;
    use rstest::rstest;
    use std::collections::HashSet;

    #[rstest]
    #[case(b">", 2)]
    #[case(b"^>v<", 4)]
    #[case(b"^v^v^v^v^v", 2)]
    fn counts_positions(#[case] input: &[u8], #[case] count: usize) {
        let iter = input.iter();
        assert_eq!(scan_positions(iter).collect::<HashSet<_>>().len(), count);
    }
}
