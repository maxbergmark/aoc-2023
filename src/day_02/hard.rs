use std::str::FromStr;

use crate::common::{error::AocError, functions::file_to_lines};

use super::common::{CubeSet, Game};

fn power(cube_set: CubeSet) -> i32 {
    cube_set.red * cube_set.green * cube_set.blue
}

fn combine_cube_sets(a: CubeSet, b: CubeSet) -> CubeSet {
    CubeSet { 
        red: i32::max(a.red, b.red), 
        green: i32::max(a.green, b.green), 
        blue: i32::max(a.blue, b.blue), 
    }
}

fn get_max_cube_set(game: Game) -> Option<CubeSet> {
    game.picks.into_iter()
        .reduce(combine_cube_sets)
}

fn sum_game_powers(filename: &str) -> Result<i32, AocError> {
    let games: Result<Vec<Game>, AocError> = file_to_lines(filename)?
        .map(|s| s.map_err(|_| AocError::FileNotFound))
        .map(|s| Game::from_str(s?.as_str()))
        .collect();

    let id_sum = games?.into_iter()
        .filter_map(get_max_cube_set)
        .map(power)
        .sum();

    Ok(id_sum)
}

#[allow(unused)]
pub fn solve() -> Result<i32, AocError> {
    sum_game_powers("input/day_02/puzzle.txt")
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_testcase() {
        assert_eq!(Ok(2286), sum_game_powers("input/day_02/easy_test.txt"))
    }

    #[test]
    fn test_solve() {
        assert_eq!(Ok(66016), solve());
    }
}
