use std::str::FromStr;

use crate::common::error::AocError;

use super::common::Almanac;


fn solve_file(filename: &str) -> Result<i64, AocError> {
    let s = std::fs::read_to_string(filename).map_err(|_| AocError::FileNotFound)?;
    let almanac = Almanac::from_str(s.as_str())?;
    let seeds = almanac.seeds.clone();
    seeds.into_iter()
        .map(|seed| almanac.seed_to_location(seed))
        .min().ok_or(AocError::ParseError)
}

// Seed 79, soil 81, fertilizer 81, water 81, light 74, temperature 78, humidity 78, location 82.
// Seed 14, soil 14, fertilizer 53, water 49, light 42, temperature 42, humidity 43, location 43.
// Seed 55, soil 57, fertilizer 57, water 53, light 46, temperature 82, humidity 82, location 86.
// Seed 13, soil 13, fertilizer 52, water 41, light 34, temperature 34, humidity 35, location 35.

pub fn solve() -> Result<i64, AocError> {
    solve_file("input/day_05/puzzle.txt")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_testcase() {
        assert_eq!(Ok(35), solve_file("input/day_05/easy_test.txt"));
    }

    #[test]
    fn test_solve() {
        assert_eq!(Ok(26273516), solve())
    }
}