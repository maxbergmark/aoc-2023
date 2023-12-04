use std::{collections::HashSet, str::FromStr};

use crate::common::{error::AocError, functions::{file_to_lines, to_parse_error}};

pub struct CardId(i32);

pub struct BingoCard {
    pub card_number: CardId,
    pub winning_numbers: HashSet<i32>,
    pub picked_numbers: HashSet<i32>,
}

fn parse_header(header: &str) -> Result<CardId, AocError> {
    header.split(' ').last()
        .and_then(|s| i32::from_str(s).ok())
        .map(CardId)
        .ok_or(AocError::ParseError)
}

fn parse_number_string(input: &str) -> Result<HashSet<i32>, AocError> {
    input.split(' ')
        .filter(|s| !s.is_empty())
        .map(i32::from_str)
        .map(to_parse_error)
        .collect()
}

fn parse_game_info(game_info: &str) -> Result<(HashSet<i32>, HashSet<i32>), AocError> {
    let parts: Vec<&str> = game_info.split(" | ").collect();
    match parts.as_slice() {
        [winning, played] => {
            Ok((parse_number_string(winning)?, parse_number_string(played)?))
        },
        _ => Err(AocError::ParseError)
    }
}

impl FromStr for BingoCard {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(": ").collect();
        match parts.as_slice() {
            [header, game_info] => {
                let card_number = parse_header(header)?;
                let (winning_numbers, picked_numbers) = parse_game_info(game_info)?;
                Ok(BingoCard { 
                    card_number, 
                    winning_numbers, 
                    picked_numbers, 
                })
            },
            _ => Err(AocError::ParseError)
        }
    }
}

pub fn get_bingo_cards(filename: &str) -> Result<Vec<BingoCard>, AocError> {
    file_to_lines(filename)?
        .flatten()
        .map(|line| BingoCard::from_str(line.as_str()))
        .collect()
}
