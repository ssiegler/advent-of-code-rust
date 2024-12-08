use aoc::*;
use std::fs::read_to_string;
use std::io::stdin;
use std::io::Read;
use std::iter::empty;
use std::path::PathBuf;

fn main() {
    let (file, day, year) = read_args();

    if let Some(Solution { year, day, wrapper }) = empty::<Solution>()
        .chain(year2015())
        .chain(year2024())
        .filter(|solution| year.is_none_or(|y: u32| y == solution.year))
        .filter(|solution| day.is_none_or(|d: u32| d == solution.day))
        .last()
    {
        let input = read_input(file.into());
        let (part1, part2) = wrapper(input);
        println!("Solutions for year {}, day {:02}:", year, day);
        println!("\t part 1: {}", part1);
        println!("\t part 2: {}", part2);
    } else {
        match (day, year) {
            (None, None) => {
                eprintln!("No solution found!");
            }
            (None, Some(year)) => {
                eprintln!("No solution found for year {}", year)
            }
            (Some(day), None) => {
                eprintln!("No solution found for day {}", day)
            }
            (Some(day), Some(year)) => {
                eprintln!("No solution found for year {}, day {:02}!", year, day);
            }
        }
    }
}

fn read_args() -> (String, Option<u32>, Option<u32>) {
    let mut args: Vec<_> = std::env::args().skip(1).take(3).collect();
    let file = args.pop().unwrap_or("-".into());
    let day = args.pop().and_then(|day| day.parse::<u32>().ok());
    let year = args.pop().and_then(|year| year.parse::<u32>().ok());
    (file, day, year)
}

fn read_input(file: PathBuf) -> String {
    if file.as_os_str() == "-" {
        let mut buffer = String::new();
        println!("Reading input until EOF:");
        stdin()
            .lock()
            .read_to_string(&mut buffer)
            .expect("Readable input on stdin");
        buffer
    } else {
        read_to_string(file).expect("Input file to be readable")
    }
}

struct Solution {
    year: u32,
    day: u32,
    wrapper: fn(String) -> (String, String),
}

macro_rules! solution {
    ($year:tt, $day:tt) => {{
        let year = stringify!($year);
        let day = stringify!($day);

        let wrapper = |data: String| {
            use $year::$day::*;

            let input = parse(&data);
            let part1 = part1(&input);
            let part2 = part2(&input);

            (part1.to_string(), part2.to_string())
        };

        Solution {
            year: year
                .trim_start_matches("year_")
                .parse::<u32>()
                .expect("integer year"),
            day: day
                .trim_start_matches("day")
                .parse::<u32>()
                .expect("integer day"),
            wrapper,
        }
    }};
}

macro_rules! solutions {
    ($year: tt, $($day:tt ),+) => {
        vec![$(solution!($year, $day)),+]
    };
}

fn year2015() -> Vec<Solution> {
    solutions!(year_2015, day01, day02, day03)
}

fn year2024() -> Vec<Solution> {
    solutions!(year_2024, day01, day02, day03, day04)
}
