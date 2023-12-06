use std::{str::FromStr, ops::Range};

use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::common::error::AocError;

use super::common::Almanac;


fn seeds_to_ranges(seeds: &[i64]) -> Result<Vec<Range<i64>>, AocError> {
    seeds.chunks(2)
        .map(|chunk| {
            match chunk {
                [from, length] => Ok(*from..*from + *length),
                _ => Err(AocError::ParseError),
            }
        })
        .collect()
}

fn find_minimum(ranges: Vec<Range<i64>>, almanac: &Almanac) -> Option<i64> {
    ranges.into_iter()
        .map(|range| {
            range
                .into_par_iter()
                .map(|seed| almanac.seed_to_location(seed))
                .min().unwrap_or(i64::MAX)
        })
        .min()
}


fn solve_file(filename: &str) -> Result<i64, AocError> {
    let s = std::fs::read_to_string(filename).map_err(|_| AocError::FileNotFound)?;
    let almanac = Almanac::from_str(s.as_str())?;
    let ranges = seeds_to_ranges(&almanac.seeds);
    let minimum = find_minimum(ranges?, &almanac);
    minimum.ok_or(AocError::SolveError)
}

#[allow(unused)]
pub fn solve() -> Result<i64, AocError> {
    solve_file("input/day_05/puzzle.txt")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_testcase() {
        assert_eq!(Ok(46), solve_file("input/day_05/easy_test.txt"));
    }

    #[test]
    #[ignore = "takes too long"]
    fn test_solve() {
        assert_eq!(Ok(34039469), solve())
    }

    #[test]
    fn solve_single() {
        let s = std::fs::read_to_string("input/day_05/easy_test.txt").unwrap();
        let almanac = Almanac::from_str(s.as_str()).unwrap();
        let res = almanac.seed_to_location(82);
        assert_eq!(46, res);
    }
}