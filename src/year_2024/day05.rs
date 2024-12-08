use std::collections::HashSet;

pub struct Input {
    rules: HashSet<(u32, u32)>,
    updates: Vec<Vec<u32>>,
}

impl Input {
    pub fn valid_updates(&self) -> impl Iterator<Item = &Vec<u32>> {
        self.updates.iter().filter(|update| self.is_valid(update))
    }

    fn is_valid(&self, update: &[u32]) -> bool {
        update.iter().enumerate().all(|(r, right)| {
            update[0..r]
                .iter()
                .all(|left| !(self.rules.contains(&(*right, *left))))
        })
    }
}

pub fn part1(input: &Input) -> u32 {
    input.valid_updates().map(|x| x[(x.len()) / 2]).sum()
}

pub fn part2(_input: &Input) -> String {
    "not yet done".to_string()
}

pub fn parse(input: &str) -> Input {
    let (rules, updates) = input
        .split_once("\n\n")
        .expect("Empty line between rule and updates");

    let rules = rules
        .lines()
        .map(|line| {
            let (l, r) = line.split_once('|').expect("Rule with '|' as separator");
            (
                l.parse::<u32>().expect("Page number"),
                r.parse::<u32>().expect("Page number"),
            )
        })
        .collect::<HashSet<(u32, u32)>>();

    let updates = updates
        .lines()
        .map(|line| {
            line.split(',')
                .map(|page| page.parse::<u32>().expect("Page number"))
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<_>>();

    Input { rules, updates }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn parse_example() {
        let parsed = parse(EXAMPLE_INPUT);
        assert_eq!(parsed.rules.len(), 21);
        assert!(parsed.rules.contains(&(47, 53)));
        assert!(parsed.rules.contains(&(53, 13)));

        assert_eq!(parsed.updates.len(), 6);
        assert_eq!(parsed.updates[0], vec![75, 47, 61, 53, 29]);
        assert_eq!(parsed.updates[5], vec![97, 13, 75, 29, 47]);
    }

    #[test]
    fn is_valid() {
        let input = parse(EXAMPLE_INPUT);

        assert!(!input.is_valid(&[75, 97, 47, 61, 53]));
    }

    #[test]
    fn validates_updates() {
        let input = parse(EXAMPLE_INPUT);
        let valid_updates = input.valid_updates().cloned().collect::<Vec<_>>();
        assert_eq!(
            valid_updates,
            vec![
                vec![75, 47, 61, 53, 29],
                vec![97, 61, 53, 29, 13],
                vec![75, 29, 13],
            ]
        );
    }

    #[test]
    fn solves_part1() {
        assert_eq!(part1(&parse(EXAMPLE_INPUT)), 143);
    }
}
