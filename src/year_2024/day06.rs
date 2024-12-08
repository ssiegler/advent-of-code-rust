use std::collections::{HashMap, HashSet};
use std::hash::Hash;

#[derive(Copy, Clone, Debug, PartialEq, Hash, Eq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

use Direction::*;

impl Direction {
    fn turn_right(&self) -> Direction {
        match self {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up,
        }
    }

    fn advance(&self, (row, column): (isize, isize)) -> (isize, isize) {
        match self {
            Up => (row - 1, column),
            Right => (row, column + 1),
            Down => (row + 1, column),
            Left => (row, column - 1),
        }
    }
}

#[derive(Clone, PartialEq)]
enum Tile {
    Empty,
    Obstacle,
}

#[derive(Clone)]
pub struct Input {
    tiles: HashMap<(isize, isize), Tile>,
    guard_start_position: (isize, isize),
    guard_start_direction: Direction,
}

fn step(
    position: (isize, isize),
    direction: Direction,
    tiles: &HashMap<(isize, isize), Tile>,
) -> Option<((isize, isize), Direction)> {
    let next_position = direction.advance(position);
    match tiles.get(&next_position) {
        None => None,
        Some(Tile::Empty) => Some((next_position, direction)),
        Some(Tile::Obstacle) => step(position, direction.turn_right(), tiles),
    }
}

impl Input {
    fn guard_positions(&self) -> HashSet<(isize, isize)> {
        self.patrol()
            .map(|(position, _)| position)
            .collect::<HashSet<_>>()
    }

    fn patrol(&self) -> impl Iterator<Item = ((isize, isize), Direction)> + use<'_> {
        std::iter::successors(
            Some((self.guard_start_position, self.guard_start_direction)),
            |(position, direction)| step(*position, *direction, &self.tiles),
        )
    }

    fn count_cycles(&mut self) -> usize {
        let mut obstacles = self.guard_positions();
        obstacles.remove(&self.guard_start_position);
        obstacles
            .iter()
            .filter(|obstacle| {
                self.tiles.insert(**obstacle, Tile::Obstacle);
                let has_cycle = {
                    let fast = self.patrol().step_by(2);
                    let slow = self.patrol();
                    fast.zip(slow).skip(1).any(|(fast, slow)| fast == slow)
                };
                self.tiles.insert(**obstacle, Tile::Empty);
                has_cycle
            })
            .count()
    }
}

pub fn part1(input: &Input) -> usize {
    input.guard_positions().len()
}

pub fn part2(input: &Input) -> usize {
    let mut clone: Input = input.clone();
    clone.count_cycles()
}

pub fn parse(input: &str) -> Input {
    let mut guard: Option<(usize, usize, Direction)> = None;
    let mut tiles = HashMap::new();
    input.lines().enumerate().for_each(|(row, line)| {
        line.chars().enumerate().for_each(|(column, char)| {
            let tile = match char {
                '#' => Tile::Obstacle,
                '^' => {
                    guard.replace((row, column, Up));
                    Tile::Empty
                }
                _ => Tile::Empty,
            };
            tiles.insert((row as isize, column as isize), tile);
        })
    });
    let (guard_start_position, guard_start_direction) = guard
        .map(|(r, c, d)| ((r as isize, c as isize), d))
        .expect("Guard on map");
    Input {
        tiles,
        guard_start_position,
        guard_start_direction,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn parses_input() {
        let input = parse(EXAMPLE_INPUT);
        assert_eq!(input.guard_start_direction, Up);
        assert_eq!(input.guard_start_position, (6, 4));
        assert_eq!(input.tiles.len(), 100);
        assert_eq!(
            input
                .tiles
                .iter()
                .filter(|(_, tile)| { **tile == Tile::Obstacle })
                .count(),
            8
        );
    }

    #[test]
    fn patrols_til_first_obstruction() {
        let input = parse(EXAMPLE_INPUT);
        let positions = input.patrol().take(6).collect::<Vec<_>>();
        assert_eq!(
            positions,
            vec![
                ((6, 4), Up),
                ((5, 4), Up),
                ((4, 4), Up),
                ((3, 4), Up),
                ((2, 4), Up),
                ((1, 4), Up)
            ]
        );
    }

    #[test]
    fn turns_left_on_obstruction() {
        let input = parse(EXAMPLE_INPUT);
        assert_eq!(input.patrol().nth(6), Some(((1, 5), Right)));
    }

    #[test]
    fn solves_part1() {
        assert_eq!(part1(&parse(EXAMPLE_INPUT)), 41);
    }
    #[test]
    fn solves_part2() {
        assert_eq!(part2(&parse(EXAMPLE_INPUT)), 6);
    }
}
