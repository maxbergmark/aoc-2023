use crate::common::{error::AocError, functions::file_to_lines};

fn check_digit_strings(line: &[u8]) -> Option<i32> {
    let patterns = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    patterns.into_iter().enumerate()
        .filter_map(|(i, pattern)| {
            if line.starts_with(pattern.as_bytes()) {
                Some(i as i32 +1)
            } else {
                None
            }
        })
        .next()
}

fn starts_with_digit(line: &[u8]) -> Option<i32> {
    let c = line.first()?;
    match c {
        b'1'..=b'9' => Some((c - b'0') as i32),
        _ => check_digit_strings(line)
    }
}

fn find_all_digits(line: &String) -> Vec<i32> {
    let line = line.as_bytes();

    line.iter().enumerate()
    .filter_map(|(i, _)| starts_with_digit(&line[i..]))
    .collect()

}

fn line_to_digit(line: String) -> Option<i32> {
    let matches = find_all_digits(&line);
    let first = matches.first()?;
    let last = matches.last()?;
    Some(10 * first + last)
}

fn sum_file(filename: &str) -> Result<i32, AocError> {
    let sum: i32 = file_to_lines(filename)?
        .flatten()
        .filter_map(line_to_digit)
        .sum();

    Ok(sum)

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
        assert_eq!(Ok(281), sum_file("input/day_01/hard_test.txt"))
    }

    #[test]
    fn test_solve() {
        assert_eq!(Ok(56324), solve());
    }

    #[rstest]
    #[case("two1nine", 29)]
    #[case("eightwothree", 83)]
    #[case("abcone2threexyz", 13)]
    #[case("xtwone3four", 24)]
    #[case("4nineeightseven2", 42)]
    #[case("zoneight234", 14)]
    #[case("7pqrstsixteen", 76)]
    fn test_cases(#[case] input: &str, #[case] expected: i32) {
        assert_eq!(Some(expected), line_to_digit(input.into()));
    }
}
