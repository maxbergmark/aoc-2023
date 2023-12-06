use crate::{common::error::AocError, day_06::common::parse_as_single};

use super::common::num_possibilities;


fn solve_file(filename: &str) -> Result<i64, AocError> {
    let race = parse_as_single(filename)?;
    Ok(num_possibilities(race))
}

pub fn solve() -> Result<i64, AocError> {
    solve_file("input/day_06/puzzle.txt")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_testcase() {
        assert_eq!(Ok(71503), solve_file("input/day_06/easy_test.txt"));
    }

    #[test]
    fn test_solve() {
        assert_eq!(Ok(42588603), solve())
    }
}