use itertools::Itertools;

type Input = Vec<(Option<usize>, u8)>;

pub fn part1(input: &Input) -> usize {
    checksum(&compact(input))
}

fn checksum(blocks: &[Option<usize>]) -> usize {
    blocks
        .iter()
        .enumerate()
        .filter_map(|(position, block)| block.map(|id| position * id))
        .sum()
}

pub fn part2(_input: &Input) -> String {
    "todo".to_string()
}

pub fn parse(input: &str) -> Input {
    let mut result = Vec::new();
    for (id, mut chunk) in (&input.bytes().chunks(2)).into_iter().enumerate() {
        if let Some(size) = chunk.next() {
            result.push((Some(id), size - b'0'));
        }
        if let Some(size) = chunk.next() {
            result.push((None, size - b'0'));
        }
    }

    result
}

fn blocks(input: &Input) -> Vec<Option<usize>> {
    input
        .iter()
        .flat_map(|(id, size)| (0u8..*size).map(move |_| id))
        .cloned()
        .collect()
}

fn compact(input: &Input) -> Vec<Option<usize>> {
    let blocks = blocks(input);
    let mut left = 0;
    let mut right = blocks.len() - 1;
    let mut result = blocks.clone();
    while left < right {
        if result[left].is_none() {
            result.swap(left, right);
        }
        if result[left].is_some() {
            left += 1;
        }
        if result[right].is_none() {
            right -= 1;
        }
    }
    result
}

#[cfg(test)]
fn print(input: &[Option<usize>]) -> String {
    let mut output = String::with_capacity(input.len());
    for block in input {
        if let Some(id) = block {
            output.push_str(&id.to_string());
        } else {
            output.push('.');
        }
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn parse_12345() {
        assert_eq!(
            parse("12345"),
            vec![
                (Some(0), 1),
                (None, 2),
                (Some(1), 3),
                (None, 4),
                (Some(2), 5)
            ]
        );
    }

    const EXAMPLE: &str = "2333133121414131402";

    #[rstest]
    #[case("12345", "022111222......")]
    #[case(EXAMPLE, "0099811188827773336446555566..............")]
    fn compacts(#[case] input: &str, #[case] expected_output: &str) {
        let input = parse(input);
        assert_eq!(print(&compact(&input)), expected_output);
    }

    #[test]
    fn solves_example_part1() {
        assert_eq!(part1(&parse(EXAMPLE)), 1928);
    }
}
