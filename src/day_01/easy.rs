use crate::common::{error::AocError, functions::file_to_lines};
use crate::common::traits::ToBaseTen;

fn parse_line(line: String) -> Option<i32> {
    let remaining: String = line.chars()
        .filter(char::is_ascii_digit)
        .collect();
    let first = remaining.chars().next().and_then(char::to_base_10)?;
    let last = remaining.chars().last().and_then(char::to_base_10)?;
    Some(10*first + last)
}

fn sum_file(filename: &str) -> Result<i32, AocError> {
    let answer: i32 = file_to_lines(filename)?
        .filter_map(|line| parse_line(line.ok()?))
        .sum();
    Ok(answer)

}

#[allow(unused)]
pub fn solve() -> Result<i32, AocError> {
    sum_file("input/day_01/puzzle.txt")
}


#[cfg(test)]
mod test {
    use rstest::rstest;

    use super::*;

    #[test]
    fn test_testcase() {
        assert_eq!(Ok(142), sum_file("input/day_01/easy_test.txt"))
    }

    #[test]
    fn test_solve() {
        assert_eq!(Ok(55108), solve());
    }

    #[rstest]
    #[case("1abc2", 12)]
    #[case("pqr3stu8vwx", 38)]
    #[case("a1b2c3d4e5f", 15)]
    #[case("treb7uchet", 77)]
    fn test_cases(#[case] input: &str, #[case] expected: i32) {
        assert_eq!(Some(expected), parse_line(input.into()));
    }

}