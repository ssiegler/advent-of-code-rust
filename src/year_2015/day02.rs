pub fn part1(presents: &[Present]) -> u32 {
    presents.iter().map(required_paper).sum()
}
pub fn part2(presents: &[Present]) -> u32 {
    presents.iter().map(required_rope).sum()
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

pub fn parse(input: &str) -> Vec<Present> {
    input
        .lines()
        .map(|line| {
            let split = line
                .split('x')
                .map(|part| part.parse::<u32>().expect("Integer dimension"))
                .collect::<Vec<_>>();
            (split[0], split[1], split[2])
        })
        .collect::<Vec<Present>>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn parses_presents() {
        assert_eq!(vec![(2, 3, 4), (1, 1, 10)], parse("2x3x4\n1x1x10"))
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
