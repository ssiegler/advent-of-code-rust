use regex::Regex;

pub enum Statement {
    Do,
    Dont,
    Mul(u32, u32),
}

pub fn part1(statements: &[Statement]) -> u32 {
    statements
        .iter()
        .filter_map(|statement| match statement {
            Statement::Mul(a, b) => Some(a * b),
            _ => None,
        })
        .sum()
}

pub fn part2(statements: &[Statement]) -> u32 {
    statements
        .iter()
        .scan(true, |enabled, statement| match statement {
            Statement::Do => {
                *enabled = true;
                Some(0)
            }
            Statement::Dont => {
                *enabled = false;
                Some(0)
            }
            Statement::Mul(a, b) if *enabled => Some(*a * *b),
            _ => Some(0),
        })
        .sum()
}

pub fn parse(input: &str) -> Vec<Statement> {
    Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)")
        .expect("valid regex")
        .captures_iter(input)
        .filter_map(|captures| match &captures.get(0).unwrap().as_str()[0..3] {
            "mul" => {
                if let [a, b] = captures
                    .iter()
                    .skip(1)
                    .flatten()
                    .map(|group| group.as_str().parse::<u32>().unwrap())
                    .take(2)
                    .collect::<Vec<_>>()[..]
                {
                    Some(Statement::Mul(a, b))
                } else {
                    None
                }
            }
            "don" => Some(Statement::Dont),
            "do(" => Some(Statement::Do),
            _ => None,
        })
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(&parse(
                "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"
            )),
            161
        );
    }
    #[test]
    fn test_part2() {
        assert_eq!(
            part2(&parse(
                "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"
            )),
            48
        );
    }
}
