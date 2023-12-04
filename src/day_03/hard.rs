use crate::common::error::AocError;

use super::common::{Schematic, Token, Digit, check_neighbors, get_schematic};

#[derive(Debug)]
enum CombinedToken {
    Number {col_start: i32, col_end: i32, number: i32},
    Symbol {row: i32, col: i32},
}

#[derive(Debug, Default)]
struct Combiner {
    number: i32,
    is_adjacent: bool,
    col_start: usize,
}

fn handle_digit(mut current: Combiner, digit: &Digit, col: usize) -> Combiner {
    if current.number == 0 {
        current.col_start = col;
    }
    if let Digit::Checked { number, is_adjacent } = digit {
        current.number = 10 * current.number + number;
        current.is_adjacent |= is_adjacent;
    }
    current
}

fn handle_non_digit(current: Combiner, col: usize) -> (Combiner, Option<CombinedToken>) {
    let new_token = if current.is_adjacent {
        Some(CombinedToken::Number { 
            col_start: current.col_start as i32,
            col_end: col as i32-1,
            number: current.number
        })
    } else {
        None
    };

    (Combiner::default(), new_token)
}

fn merge_token(mut current: Combiner, token: Token, row: usize, col: usize) -> (Combiner, Vec<CombinedToken>) {
    let mut combined_tokens = vec![];
    match &token {
        Token::Number(digit) => {
            current = handle_digit(current, digit, col);
        },
        Token::Symbol(c) => {
            if *c == '*' {
                combined_tokens.push(CombinedToken::Symbol { row: row as i32, col: col as i32 });
            }
        },
        Token::Empty => {},
    }

    if let Token::Symbol(_) | Token::Empty = token {
        let new_token;
        (current, new_token) = handle_non_digit(current, col);
        if let Some(t) = new_token {
            combined_tokens.push(t);
        }
    };
    (current, combined_tokens)
}

fn merge_row(tokens: Vec<Token>, row: usize) -> Vec<CombinedToken> {
    let mut combined_tokens = vec![];
    let mut current = Combiner::default();
    let row_length = tokens.len() as i32;

    for (col, token) in tokens.into_iter().enumerate() {
        let new_tokens;
        (current, new_tokens) = merge_token(current, token, row, col);
        combined_tokens.extend(new_tokens);
    }

    if current.is_adjacent {
        let token = CombinedToken::Number { 
            col_start: current.col_start as i32, 
            col_end: row_length-1, 
            number: current.number 
        };
        combined_tokens.push(token);
    }

    combined_tokens
}

fn merge_to_tokens(schematic: Schematic) -> Vec<Vec<CombinedToken>> {
    let mut combined_tokens = vec![];
    for (row, tokens) in schematic.tokens.into_iter().enumerate() {
        combined_tokens.push(merge_row(tokens, row))
    }
    combined_tokens
}

fn is_adjacent(token: &CombinedToken, symbol_col: i32) -> bool {
    match token {
        CombinedToken::Number { col_start, col_end, .. } => {
            (col_start - symbol_col).abs() <= 1 || (col_end - symbol_col).abs() <= 1
        },
        CombinedToken::Symbol { .. } => false,
    }
}

fn find_neighbors(tokens: &[CombinedToken], symbol_col: i32) -> Vec<&CombinedToken> {

    tokens.iter()
        .filter(|token| is_adjacent(token, symbol_col))
        .collect()
}

fn find_all_neighbors<'a>(combined_tokens: &'a[&[CombinedToken]], symbol_row: i32, symbol_col: i32) -> Vec<&'a CombinedToken> {
    let first_row = i32::max(0, symbol_row-1) as usize;

    combined_tokens.iter()
        .skip(first_row)
        .take(3)
        .flat_map(|tokens| find_neighbors(tokens, symbol_col))
        .collect()
}

fn handle_symbol(combined_tokens: &[&[CombinedToken]], row: &i32, col: &i32) -> Option<i32> {
    let neighbors = find_all_neighbors(combined_tokens, *row, *col);
    let slice = neighbors.as_slice();

    if let [first, second] = slice {
        match (first, second) {
            (CombinedToken::Number {number: first, .. }, CombinedToken::Number { number: second, .. }) => {
                Some(first * second)
            },
            _ => None,
        }
    } else {
        None
    }
}

fn check_token(token: &CombinedToken, combined_tokens: &[&[CombinedToken]]) -> i32 {
    match token {
        CombinedToken::Number { .. } => 0,
        CombinedToken::Symbol { row, col } => {
            handle_symbol(combined_tokens, row, col).unwrap_or(0)
        },
    }
}

fn check_cogs(combined_tokens: &[&[CombinedToken]]) -> i32 {
    let mut sum = 0;
    for tokens in combined_tokens.iter() {
        for token in tokens.iter() {
            sum += check_token(token, combined_tokens)
        }
    }
    sum
}

fn get_schematic_cog_sum(schematic: Schematic) -> i32 {
    let schematic = check_neighbors(schematic);
    let combined_tokens = merge_to_tokens(schematic);
    let combined_tokens: Vec<&[CombinedToken]> = combined_tokens.iter().map(Vec::as_slice).collect();
    check_cogs(combined_tokens.as_slice())
}

fn solve_file(filename: &str) -> Result<i32, AocError> {
    let schematic = get_schematic(filename)?;
    Ok(get_schematic_cog_sum(schematic))
}

#[allow(unused)]
pub fn solve() -> Result<i32, AocError> {
    solve_file("input/day_03/puzzle.txt")
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_testcase() {
        assert_eq!(Ok(467835), solve_file("input/day_03/easy_test.txt"))
    }

    #[test]
    fn test_solve() {
        assert_eq!(Ok(81296995), solve());
    }
}

