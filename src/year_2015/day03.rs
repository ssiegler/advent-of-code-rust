use std::collections::HashSet;

pub fn parse(input: &str) -> Vec<(i32, i32)> {
    input
        .bytes()
        .filter_map(|b| match b {
            b'>' => Some((0, 1)),
            b'<' => Some((0, -1)),
            b'^' => Some((1, 0)),
            b'v' => Some((-1, 0)),
            _ => None,
        })
        .collect()
}

pub fn part1(movements: &[(i32, i32)]) -> usize {
    let mut positions = HashSet::new();
    positions.insert((0, 0));
    collect_positions(&mut positions, movements.iter().cloned());
    positions.len()
}

pub fn part2(movements: &[(i32, i32)]) -> usize {
    let mut positions = HashSet::new();
    positions.insert((0, 0));
    collect_positions(&mut positions, movements.iter().step_by(2).cloned());
    collect_positions(&mut positions, movements[1..].iter().step_by(2).cloned());
    positions.len()
}

fn collect_positions(positions: &mut HashSet<(i32, i32)>, iter: impl Iterator<Item = (i32, i32)>) {
    iter.scan((0, 0), |(x, y), (dx, dy)| {
        *x += dx;
        *y += dy;
        Some((*x, *y))
    })
    .for_each(|item| {
        positions.insert(item);
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(">", 2)]
    #[case("^>v<", 4)]
    #[case("^v^v^v^v^v", 2)]
    fn counts_positions(#[case] input: &str, #[case] count: usize) {
        assert_eq!(part1(&parse(input)), count);
    }
}
