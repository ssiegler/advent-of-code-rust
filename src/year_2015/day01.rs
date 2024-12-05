use std::ops::{AddAssign, SubAssign};

pub fn part1(floors: &[i32]) -> i32 {
    floors.last().cloned().unwrap_or(0)
}

pub fn part2(floors: &[i32]) -> usize {
    floors.iter().position(|floor| *floor == -1).unwrap() + 1
}

pub fn parse(input: &str) -> Vec<i32> {
    input
        .bytes()
        .scan(0, |floor, b| {
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
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("(())", 0)]
    #[case("()()", 0)]
    #[case("(((", 3)]
    #[case("(()(()(", 3)]
    #[case("))(((((", 3)]
    #[case("())", -1)]
    #[case("))(", -1)]
    #[case(")))", -3)]
    #[case(")())())", -3)]
    fn solves_part_1_examples(#[case] input: &str, #[case] floor: i32) {
        assert_eq!(part1(&parse(input)), floor);
    }

    #[rstest]
    #[case(")", 1)]
    #[case("()())", 5)]
    fn solves_part2_examples(#[case] input: &str, #[case] position: usize) {
        assert_eq!(part2(&parse(input)), position);
    }
}
