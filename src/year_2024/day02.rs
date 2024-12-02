use crate::parser::integer;
use itertools::Itertools;
use nom::character::complete::{multispace1, newline, space1};
use nom::combinator::{all_consuming, opt};
use nom::multi::separated_list1;
use nom::sequence::terminated;

pub(super) fn solution(input: &[u8]) -> anyhow::Result<(String, String)> {
    let reports = parse(input)?;
    let safe_report_count = reports.iter().filter(is_safe).count();
    Ok((safe_report_count.to_string(), "".to_string()))
}

type Report = Vec<u32>;

fn parse(input: &[u8]) -> anyhow::Result<Vec<Report>> {
    let (_, reports) = all_consuming(terminated(
        separated_list1(newline, separated_list1(space1, integer)),
        opt(newline),
    ))(input)
    .map_err(|e| e.to_owned())?;
    Ok(reports)
}

fn is_safe(report: &&Report) -> bool {
    let (increasing, diff): (Vec<bool>, Vec<u32>) = report
        .windows(2)
        .map(|window| (window[0] < window[1], window[0].abs_diff(window[1])))
        .unzip();
    increasing.iter().all_equal() && diff.iter().all(|diff| (1..=3).contains(diff))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn parses_example() -> anyhow::Result<()> {
        let reports: Vec<Report> = vec![
            vec![7, 6, 4, 2, 1],
            vec![1, 2, 7, 8, 9],
            vec![9, 7, 6, 2, 1],
            vec![1, 3, 2, 4, 5],
            vec![8, 6, 4, 4, 1],
            vec![1, 3, 6, 7, 9],
        ];
        assert_eq!(
            reports,
            parse(
                b"\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
"
            )?
        );
        Ok(())
    }

    #[rstest]
    #[case(vec![7, 6, 4, 2, 1], true)]
    #[case(vec![1, 2, 7, 8, 9], false)]
    #[case(vec![9, 7, 6, 2, 1], false)]
    #[case(vec![1, 3, 2, 4, 5], false)]
    #[case(vec![8, 6, 4, 4, 1], false)]
    #[case(vec![1, 3, 6, 7, 9], true)]
    fn check_safety(#[case] report: Report, #[case] safe: bool) {
        assert_eq!(is_safe(&&report), safe);
    }
}
