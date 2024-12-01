use std::ops::{AddAssign, SubAssign};

pub(super) fn solution(input: &[u8]) -> anyhow::Result<(String, String)> {
    Ok((
        final_floor(input).to_string(),
        basement_position(input).to_string(),
    ))
}

fn final_floor(input: &[u8]) -> i32 {
    scan_floors(input).last().unwrap_or(0)
}

fn basement_position(input: &[u8]) -> usize {
    scan_floors(input).position(|floor| floor == -1).unwrap() + 1
}

fn scan_floors(input: &[u8]) -> impl Iterator<Item = i32> + use<'_> {
    input.iter().scan(0, |floor, b| {
        match b {
            b'(' => {
                floor.add_assign(1);
            }
            b')' => {
                floor.sub_assign(1);
            }
            _ => {}
        };
        Some(*floor)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(b"(())", 0)]
    #[case(b"()()", 0)]
    #[case(b"(((", 3)]
    #[case(b"(()(()(", 3)]
    #[case(b"))(((((", 3)]
    #[case(b"())", -1)]
    #[case(b"))(", -1)]
    #[case(b")))", -3)]
    #[case(b")())())", -3)]
    fn solves_part_1_examples(#[case] input: &[u8], #[case] floor: i32) {
        assert_eq!(final_floor(input), floor);
    }

    #[rstest]
    #[case(b")", 1)]
    #[case(b"()())", 5)]
    fn solves_part2_examples(#[case] input: &[u8], #[case] position: usize) {
        assert_eq!(basement_position(input), position);
    }
}
