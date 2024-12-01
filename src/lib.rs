use anyhow::{anyhow, Result};

pub type Solution = fn(&[u8]) -> Result<(String, String)>;

mod year_2015;
mod year_2024;

static SOLUTIONS: &[&[Solution]] = &[
    year_2015::SOLUTIONS,
    &[], // 2016
    &[], // 2017
    &[], // 2018
    &[], // 2019
    &[], // 2020
    &[], // 2021
    &[], // 2022
    &[], // 2023
    year_2024::SOLUTIONS,
];

pub fn find_solution(day: u8, year: u16) -> Result<&'static Solution> {
    SOLUTIONS
        .get(year as usize - 2015)
        .and_then(|solutions: &&[Solution]| solutions.get(day as usize - 1))
        .ok_or_else(|| anyhow!("No solution for day {} of year {}", day, year))
}
