pub fn part1(reports: &[impl AsRef<[i32]>]) -> usize {
    reports
        .iter()
        .filter(|report| is_safe(report.as_ref()))
        .count()
}

pub fn part2(reports: &[impl AsRef<[i32]>]) -> usize {
    reports
        .iter()
        .filter(|report| is_safe_with_dampener(report.as_ref()))
        .count()
}

pub fn parse(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse::<i32>().expect("Numbers separated by whitespace"))
                .collect()
        })
        .collect()
}

fn is_safe(report: &[i32]) -> bool {
    report
        .windows(2)
        .map(|window| window[1] - window[0])
        .filter(|difference| difference.abs() <= 3)
        .map(|difference| difference.signum())
        .sum::<i32>()
        .abs()
        == (report.len() - 1) as i32
}
fn is_safe_with_dampener(report: &[i32]) -> bool {
    (0..report.len()).any(|n| {
        let mut dampened: Vec<i32> = report.to_vec();
        dampened.remove(n);
        is_safe(&dampened)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    const EXAMPLE_INPUT: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

    #[test]
    fn parses_example() {
        let reports: Vec<Vec<i32>> = vec![
            vec![7, 6, 4, 2, 1],
            vec![1, 2, 7, 8, 9],
            vec![9, 7, 6, 2, 1],
            vec![1, 3, 2, 4, 5],
            vec![8, 6, 4, 4, 1],
            vec![1, 3, 6, 7, 9],
        ];
        assert_eq!(reports, parse(EXAMPLE_INPUT));
    }

    #[rstest]
    #[case(vec![7, 6, 4, 2, 1], true)]
    #[case(vec![1, 2, 7, 8, 9], false)]
    #[case(vec![9, 7, 6, 2, 1], false)]
    #[case(vec![1, 3, 2, 4, 5], false)]
    #[case(vec![8, 6, 4, 4, 1], false)]
    #[case(vec![1, 3, 6, 7, 9], true)]
    fn check_safety(#[case] report: Vec<i32>, #[case] safe: bool) {
        assert_eq!(is_safe(&report), safe);
    }
    #[rstest]
    #[case(vec![7, 6, 4, 2, 1], true)]
    #[case(vec![1, 2, 7, 8, 9], false)]
    #[case(vec![9, 7, 6, 2, 1], false)]
    #[case(vec![1, 3, 2, 4, 5], true)]
    #[case(vec![8, 6, 4, 4, 1], true)]
    #[case(vec![1, 3, 6, 7, 9], true)]
    fn check_safety_with_dampener(#[case] report: Vec<i32>, #[case] safe: bool) {
        assert_eq!(is_safe_with_dampener(&report), safe);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(EXAMPLE_INPUT)), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(EXAMPLE_INPUT)), 4);
    }
}
