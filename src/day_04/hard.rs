
use std::collections::HashSet;

use crate::common::error::AocError;

use super::common::{BingoCard, get_bingo_cards};

fn get_score(bingo_card: &BingoCard) -> i32 {
    let intersection: HashSet<&i32> = bingo_card.winning_numbers
        .intersection(&bingo_card.picked_numbers).collect();
    intersection.len() as i32
}

fn calculate_card_copies(bingo_cards: Vec<BingoCard>) -> i32 {
    let n = bingo_cards.len();
    let scores: Vec<i32> = bingo_cards.iter()
        .map(get_score)
        .collect();

    let mut copies = vec![1; n];
    for i in 0..n {
        let n_copy = copies[i];
        copies.iter_mut()
            .skip(i+1)
            .take(scores[i] as usize)
            .for_each(|item| *item += n_copy);
    }
    copies.iter().sum()
}

fn solve_file(filename: &str) -> Result<i32, AocError> {
    let bingo_cards = get_bingo_cards(filename)?;
    let total_score = calculate_card_copies(bingo_cards);
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
        assert_eq!(Ok(30), solve_file("input/day_04/easy_test.txt"))
    }

    #[test]
    fn test_solve() {
        assert_eq!(Ok(7013204), solve());
    }
}

