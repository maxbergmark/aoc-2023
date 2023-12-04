use crate::common::error::AocError;

use super::common::{Schematic, Token, Digit, check_neighbors, get_schematic};

#[derive(Debug, Default)]
struct Combiner {
    number: i32,
    is_adjacent: bool,
}

fn handle_digit(mut current: Combiner, digit: &Digit) -> Combiner {
    if let Digit::Checked { number, is_adjacent } = digit {
        current.number = 10 * current.number + number;
        current.is_adjacent |= is_adjacent;
    }
    current
}

fn handle_non_digit(current: &Combiner) -> Option<i32> {
    if current.is_adjacent {
        Some(current.number)
    } else {
        None
    }
}

fn handle_token(mut current: Combiner, token: Token) -> (Combiner, Option<i32>) {
    match token {
        Token::Number(digit) => {
            current = handle_digit(current, &digit);
            (current, None)
        },
        Token::Symbol(_) | Token::Empty => {
            (Combiner::default(), handle_non_digit(&current))
        },
    }
}

fn merge_row_neighbors(tokens: Vec<Token>) -> Vec<i32> {

    let (current, mut nums) = tokens.into_iter().fold(
        (Combiner::default(), vec![]), 
        |(mut current, mut nums), token| {
            let number;
            (current, number) = handle_token(current, token);
            if let Some(n) = number {
                nums.push(n);
            }
            (current, nums)
        }
    );

    // if there's a token at the end of the row
    if current.is_adjacent {
        nums.push(current.number);
    }

    nums
}

fn merge_all_row_neighbors(schematic: Schematic) -> Vec<i32> {
    schematic.tokens.into_iter()
        .flat_map(merge_row_neighbors)
        .collect()
}

fn get_schematic_sum(schematic: Schematic) -> i32 {
    let schematic = check_neighbors(schematic);
    let numbers = merge_all_row_neighbors(schematic);
    numbers.into_iter().sum()
}

fn solve_file(filename: &str) -> Result<i32, AocError> {
    let schematic = get_schematic(filename)?;
    Ok(get_schematic_sum(schematic))
}

#[allow(unused)]
pub fn solve() -> Result<i32, AocError> {
    solve_file("input/day_03/puzzle.txt")
}


#[cfg(test)]
mod test {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_testcase() {
        assert_eq!(Ok(4361), solve_file("input/day_03/easy_test.txt"))
    }

    #[test]
    fn test_merge_tokens() {
        let schematic = get_schematic("input/day_03/easy_test.txt").unwrap();
        let schematic = check_neighbors(schematic);
        let numbers = merge_all_row_neighbors(schematic);
        let expected = vec![467, 35, 633, 617, 592, 755, 664, 598];
        assert_eq!(expected, numbers);
    }

    #[test]
    fn test_puzzle_tokens() {
        let schematic = get_schematic("input/day_03/puzzle.txt").unwrap();
        let schematic = check_neighbors(schematic);
        let numbers = merge_all_row_neighbors(schematic);
        let expected = vec![507, 961, 668, 189, 906, 805, 130, 880, 684, 17, 65, 91, 464, 208, 260, 967, 38, 692, 676, 247, 652, 585];
        let numbers = &numbers.as_slice()[..expected.len()];
        assert_eq!(expected, numbers);
    }

    #[test]
    fn test_manual() {
        let input = "123\n4*6\n789";
        let schematic = Schematic::from_str(input).unwrap();
        let schematic = check_neighbors(schematic);
        let numbers = merge_all_row_neighbors(schematic);
        let expected = vec![123, 4, 6, 789];
        assert_eq!(expected, numbers);
    }

    #[test]
    fn test_solve() {
        assert_eq!(Ok(517021), solve());
    }
}

