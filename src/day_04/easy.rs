
use std::collections::HashSet;

use crate::common::error::AocError;

use super::common::{BingoCard, get_bingo_cards};

fn matches_to_score(n: i32) -> i32 {
    if n > 0 {
        2i32.pow(n as u32 - 1)        
    } else {
        0
    }
}

fn get_score(bingo_card: &BingoCard) -> i32 {
    let intersection: HashSet<&i32> = bingo_card.winning_numbers
        .intersection(&bingo_card.picked_numbers).collect();
    let num_winning = intersection.len() as i32;
    matches_to_score(num_winning)
}

fn sum_total_score(bingo_cards: Vec<BingoCard>) -> i32 {
    bingo_cards.iter()
        .map(get_score)
        .sum()
}

fn solve_file(filename: &str) -> Result<i32, AocError> {
    let bingo_cards = get_bingo_cards(filename)?;
    let total_score = sum_total_score(bingo_cards);
    Ok(total_score)
}

#[allow(unused)]
pub fn solve() -> Result<i32, AocError> {
    solve_file("input/day_04/puzzle.txt")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_testcase() {
        assert_eq!(Ok(13), solve_file("input/day_04/easy_test.txt"))
    }

    #[test]
    fn test_solve() {
        assert_eq!(Ok(22488), solve());
    }
}

