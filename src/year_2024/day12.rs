use std::collections::{HashMap, HashSet, VecDeque};

type Input = HashMap<(isize, isize), u8>;
pub fn part1(input: &Input) -> usize {
    total_price(input, price)
}

fn total_price(input: &Input, region_price: fn(&HashSet<(isize, isize)>) -> usize) -> usize {
    let mut visited = HashSet::new();
    let mut total = 0;
    for position in input.keys() {
        if visited.insert(*position) {
            let region = find_region(input, position);
            visited.extend(&region);
            total += region_price(&region)
        }
    }
    total
}

fn find_region(map: &Input, &position @ (x, y): &(isize, isize)) -> HashSet<(isize, isize)> {
    let mut region = HashSet::new();
    let inside = {
        let plant = *map.get(&position).expect("plant for position");
        move |x, y| map.get(&(x, y)).map(|p| *p == plant).unwrap_or(false)
    };

    let mut queue = VecDeque::new();
    queue.push_back((x, x, y, 1));
    queue.push_back((x, x, y - 1, -1));

    while let Some((mut x1, x2, y, dy)) = queue.pop_front() {
        let mut x = x1;
        if inside(x, y) && !region.contains(&(x, y)) {
            while inside(x - 1, y) && region.insert((x - 1, y)) {
                x -= 1;
            }
            if x < x1 {
                queue.push_back((x, x1 - 1, y - dy, -dy));
            }
        }
        while x1 <= x2 {
            while inside(x1, y) && region.insert((x1, y)) {
                x1 += 1;
            }
            if x1 > x {
                queue.push_back((x, x1 - 1, y + dy, dy));
            }
            if x1 - 1 > x2 {
                queue.push_back((x2 + 1, x1 - 1, y - dy, -dy));
            }
            x1 += 1;
            while (region.contains(&(x1, y)) || !inside(x1, y)) && x1 < x2 {
                x1 += 1;
            }
            x = x1;
        }
    }
    region
}

fn price(region: &HashSet<(isize, isize)>) -> usize {
    let area = region.len();
    let perimeter = region
        .iter()
        .flat_map(neighbors)
        .filter(|position| !region.contains(position))
        .count();
    area * perimeter
}

fn discounted_price(region: &HashSet<(isize, isize)>) -> usize {
    let area = region.len();
    let sides = region
        .iter()
        .map(|position| count_corners(region, position))
        .sum::<usize>();
    area * sides
}

fn count_corners(region: &HashSet<(isize, isize)>, &(x, y): &(isize, isize)) -> usize {
    [(-1, -1), (-1, 1), (1, 1), (1, -1)]
        .iter()
        .filter(|(dx, dy)| {
            !region.contains(&(x + *dx, y))
                && (!region.contains(&(x, y + *dy)) || region.contains(&(x + *dx, y + *dy)))
        })
        .count()
}

fn neighbors(&(x, y): &(isize, isize)) -> impl Iterator<Item = (isize, isize)> {
    [(x - 1, y), (x, y - 1), (x + 1, y), (x, y + 1)].into_iter()
}

pub fn part2(input: &Input) -> usize {
    total_price(input, discounted_price)
}

pub fn parse(input: &str) -> Input {
    input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.bytes()
                .enumerate()
                .map(move |(column, plant)| ((row as isize, column as isize), plant))
        })
        .collect::<HashMap<_, _>>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    const EXAMPLE1: &str = "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";

    const EXAMPLE2: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

    #[rstest]
    #[case(EXAMPLE1, 772)]
    #[case(EXAMPLE2, 1930)]
    fn solves_part1(#[case] input: &str, #[case] price: usize) {
        assert_eq!(part1(&parse(input)), price);
    }

    #[rstest]
    #[case(
        "AAAA
BBCD
BBCC
EEEC",
        80
    )]
    #[case(EXAMPLE1, 436)]
    #[case(
        "EEEEE
EXXXX
EEEEE
EXXXX
EEEEE
",
        236
    )]
    #[case(
        "AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA
",
        368
    )]
    #[case(EXAMPLE2, 1206)]
    fn solves_part2(#[case] input: &str, #[case] price: usize) {
        assert_eq!(part2(&parse(input)), price);
    }

    #[rstest]
    #[case(&[(0,0)], (0,0), 4)]
    fn counts_counters(
        #[case] region: &[(isize, isize)],
        #[case] position: (isize, isize),
        #[case] corners: usize,
    ) {
        let region: HashSet<(isize, isize)> = region.iter().cloned().collect();
        assert_eq!(count_corners(&region, &position), corners);
    }
}
