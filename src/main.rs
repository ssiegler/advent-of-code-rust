use advent_of_code::find_solution;
use anyhow::{Context, Result};
use chrono::Datelike;
use chrono::Local;
use clap::Parser;
use fs::read;
use io::stdin;
use std::fs;
use std::io;
use std::io::Read;
use std::path::PathBuf;

/// Apply the solution for the given year and day to the puzzle input.
#[derive(Parser)]
#[command()]
struct Cli {
    /// The day from 1 to 25 of the solution to apply. Assumes today if missing.
    #[arg(long, value_parser = clap::value_parser!(u8).range(1..=25))]
    day: Option<u8>,

    /// The year of the solution to apply. Assumes the current year if missing.
    #[arg(long, requires = "day", value_parser = clap::value_parser!(u16).range(2015..))]
    year: Option<u16>,

    /// The file to read puzzle input from. To read from stdin, use `-`.
    #[arg()]
    file: PathBuf,
}

fn main() -> Result<()> {
    let Cli { day, year, file } = Cli::parse();
    let (day, year) = default_to_today(day, year);
    let solution = find_solution(day, year)?;
    let input = read_input(file)?;
    let (part1, part2) = solution(&input)?;
    println!("Solutions for year {}, day {:02}:", year, day);
    println!("\t part 1: {}", part1);
    println!("\t part 2: {}", part2);
    Ok(())
}

fn default_to_today(day: Option<u8>, year: Option<u16>) -> (u8, u16) {
    if let (Some(day), Some(year)) = (day, year) {
        (day, year)
    } else {
        let today = Local::now().date_naive();
        (
            today.day().try_into().expect("day to fit into u8"),
            today.year().try_into().expect("year to fit into u16"),
        )
    }
}

fn read_input(file: PathBuf) -> Result<Vec<u8>> {
    Ok(if file.as_os_str() == "-" {
        let mut buffer = Vec::new();
        stdin().lock().read_to_end(&mut buffer)?;
        buffer
    } else {
        read(&file).with_context(|| format!("Reading file: '{}'", file.display()))?
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        Cli::command().debug_assert();
    }
}
