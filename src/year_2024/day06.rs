use std::collections::HashSet;

#[derive(Copy, Clone, Debug, PartialEq)]
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
}

pub struct Input {
    width: usize,
    height: usize,
    obstructions_positions: HashSet<(usize, usize)>,
    guard_start_position: (usize, usize),
    guard_start_direction: Direction,
}

impl Input {
    fn advance(
        &self,
        (row, column): &(usize, usize),
        direction: &Direction,
    ) -> Option<(usize, usize)> {
        let (row, column, direction) = (*row, *column, *direction);
        match direction {
            Up if row > 0 => Some((row - 1, column)),
            Right if column < self.width - 1 => Some((row, column + 1)),
            Down if row < self.height - 1 => Some((row + 1, column)),
            Left if column > 0 => Some((row, column - 1)),
            _ => None,
        }
    }

    fn patrol(&self) -> impl Iterator<Item = ((usize, usize), Direction)> + use<'_> {
        std::iter::successors(
            Some((self.guard_start_position, self.guard_start_direction)),
            |(position, direction)| {
                self.advance(position, direction).and_then(|advanced| {
                    if self.obstructions_positions.contains(&advanced) {
                        let turned = direction.turn_right();
                        self.advance(position, &turned)
                            .map(|position| (position, turned))
                    } else {
                        Some((advanced, *direction))
                    }
                })
            },
        )
    }
}

pub fn part1(input: &Input) -> usize {
    input
        .patrol()
        .map(|(position, _)| position)
        .collect::<HashSet<_>>()
        .len()
}

pub fn part2(_input: &Input) -> String {
    "not yet done".to_string()
}

pub fn parse(input: &str) -> Input {
    let height = input.lines().count();
    let width = input.find('\n').expect("Multiple lines");
    let mut guard: Option<(usize, usize, Direction)> = None;
    let mut obstructions_positions = HashSet::new();
    input.lines().enumerate().for_each(|(row, line)| {
        line.chars()
            .enumerate()
            .for_each(|(column, char)| match char {
                '#' => {
                    obstructions_positions.insert((row, column));
                }
                '^' => {
                    guard.replace((row, column, Up));
                }
                '>' => {
                    guard.replace((row, column, Right));
                }
                'v' => {
                    guard.replace((row, column, Down));
                }
                '<' => {
                    guard.replace((row, column, Left));
                }
                _ => {}
            })
    });
    let (guard_start_position, guard_start_direction) =
        guard.map(|(r, c, d)| ((r, c), d)).expect("Guard on map");
    Input {
        width,
        height,
        obstructions_positions,
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
        assert_eq!(input.height, 10);
        assert_eq!(input.width, 10);
        assert_eq!(input.obstructions_positions.len(), 8);
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
}
