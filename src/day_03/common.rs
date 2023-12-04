use std::str::FromStr;

use crate::common::{error::AocError, traits::ToBaseTen};


#[derive(Debug)]
pub struct Schematic {
    pub tokens: Vec<Vec<Token>>,
}

#[derive(Debug, PartialEq)]
pub enum Digit {
    Unchecked { number: i32 },
    Checked { number: i32, is_adjacent: bool },
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Number(Digit),
    Symbol(char),
    Empty,
}

fn line_to_token_vec(line: &str) -> Vec<Token> {
    line.chars()
        .map(|ch| ch.into())
        .collect()
}

impl FromStr for Schematic {
    type Err = AocError;


    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens: Vec<Vec<Token>> = s.lines()
            .map(line_to_token_vec)
            .collect();

        Ok(Schematic { tokens })
    }
}

impl FromStr for Token {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 1 {
            Err(AocError::ParseError)
        } else {
            let char = s.chars().next().ok_or(AocError::ParseError)?;
            match char {
                '0'..='9' => {
                    let digit = char.to_base_10().ok_or(AocError::ParseError)?;
                    Ok(Token::Number(Digit::Unchecked { number: digit }))
                },
                '.' => Ok(Token::Empty),
                _ => Ok(Token::Symbol(char)),
            }
        }
    }
}

impl From<char> for Token {
    fn from(value: char) -> Self {
        match value {
            '0'..='9' => {
                let digit = value.to_base_10().unwrap();
                Token::Number(Digit::Unchecked { number: digit })
            },
            '.' => Token::Empty,
            c => Token::Symbol(c),
        }
}
}


fn get_neighbors(schematic: &Schematic, row: i32, col: i32) -> Vec<&Token> {
    let mut neighbors = vec![];
    let m = schematic.tokens.len() as i32;
    let n = schematic.tokens.get(0).map(|v| v.len()).unwrap_or(0) as i32;

    for x in -1..=1 {
        for y in -1..=1 {
            if x == 0 && y == 0 {
                continue;
            }
            if row + y < 0 || col + x < 0 {
                continue;
            }
            if row + y >= m || col + x >= n {
                continue;
            }
            neighbors.push(&schematic.tokens[(row+y) as usize][(col+x) as usize])
        }
    }
    neighbors
}

fn has_symbol_neighbor(schematic: &Schematic, row: i32, col: i32) -> bool {
    get_neighbors(schematic, row, col).into_iter()
        .any(|token| match token {
            Token::Symbol(_) => true,
            Token::Number(_) | Token::Empty => false,
        })

}

pub struct Change(pub usize, pub usize, pub Token);


pub fn check_token(schematic: &Schematic, token: &Token, changes: &mut Vec<Change>, row: usize, col: usize) {
    match token {
        Token::Number(digit) => {
            let is_adjacent = has_symbol_neighbor(schematic, row as i32, col as i32);
            let new_digit = match digit {
                Digit::Unchecked { number } => Digit::Checked { number: *number, is_adjacent },
                Digit::Checked { .. } => unreachable!("Only parse digits once"),
            };
            changes.push(Change(row, col, Token::Number(new_digit)));
        },
        Token::Symbol(_) => {},
        Token::Empty => {},
    }
}

pub fn check_neighbors(schematic: Schematic) -> Schematic {
    let mut changes = vec![];
    for (row, tokens) in schematic.tokens.iter().enumerate() {
        for (col, token) in tokens.iter().enumerate() {
            check_token(&schematic, token, &mut changes, row, col);
        }
    }

    let mut schematic = schematic;
    for Change(row, col, token) in changes {
        schematic.tokens[row][col] = token;
    }
    schematic
}

pub fn get_schematic(filename: &str) -> Result<Schematic, AocError> {
    let input = std::fs::read_to_string(filename).map_err(|_| AocError::FileNotFound)?;
    Schematic::from_str(input.as_str())
}

