use crate::{common::error::AocError, day_06::common::{parse, num_possibilities}};

fn solve_file(filename: &str) -> Result<i64, AocError> {
    let races = parse(filename)?;
    let possibilities: i64 = races.into_iter()
        .map(num_possibilities)
        .product();
    Ok(possibilities)
}

pub fn solve() -> Result<i64, AocError> {
    solve_file("input/day_06/puzzle.txt")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_testcase() {
        assert_eq!(Ok(288), solve_file("input/day_06/easy_test.txt"));
    }

    #[test]
    fn test_solve() {
        assert_eq!(Ok(345015), solve())
    }
}