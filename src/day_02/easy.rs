use std::str::FromStr;

use crate::common::{error::AocError, functions::file_to_lines};

use super::common::{CubeSet, Game};


fn is_valid(cube_set: &CubeSet, truth: &CubeSet) -> bool {
    cube_set.red <= truth.red
    && cube_set.green <= truth.green
    && cube_set.blue <= truth.blue
}

fn is_game_valid(game: &Game, truth: &CubeSet) -> bool {
    let comp = |cube_set| is_valid(cube_set, truth); 
    game.picks.iter().all(comp)
}

fn sum_possible_game_ids(filename: &str) -> Result<i32, AocError> {
    let max_allowed = CubeSet {
        red: 12,
        green: 13,
        blue: 14,
    };

    let games: Result<Vec<Game>, AocError> = file_to_lines(filename)?
        .map(|s| s.map_err(|_| AocError::FileNotFound))
        .map(|s| Game::from_str(s?.as_str()))
        .collect();

    let id_sum = games?.into_iter()
        .filter(|game| is_game_valid(game, &max_allowed))
        .map(|game| game.id.0)
        .sum();

    // println!("games: {games:?}");
    Ok(id_sum)
}

#[allow(unused)]
pub fn solve() -> Result<i32, AocError> {
    sum_possible_game_ids("input/day_02/puzzle.txt")
}

#[cfg(test)]
mod test {
    use rstest::rstest;

    use super::*;
    use super::super::common::GameId;

    #[test]
    fn test_testcase() {
        assert_eq!(Ok(8), sum_possible_game_ids("input/day_02/easy_test.txt"))
    }

    #[test]
    fn test_solve() {
        assert_eq!(Ok(2541), solve());
    }

    #[rstest]
    #[case(
        "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", 
        Game { id: GameId(1), picks: vec![
            CubeSet { red: 4, green: 0, blue: 3 }, 
            CubeSet { red: 1, green: 2, blue: 6 }, 
            CubeSet { red: 0, green: 2, blue: 0 }]
        })]
    #[case(
        "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue", 
        Game { id: GameId(2), picks: vec![
            CubeSet { red: 0, green: 2, blue: 1 }, 
            CubeSet { red: 1, green: 3, blue: 4 }, 
            CubeSet { red: 0, green: 1, blue: 1 }]
        })]
    #[case(
        "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red", 
        Game { id: GameId(3), picks: vec![
            CubeSet { red: 20, green: 8, blue: 6 }, 
            CubeSet { red: 4, green: 13, blue: 5 }, 
            CubeSet { red: 1, green: 5, blue: 0 }]
        })]
    #[case(
        "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red", 
        Game { id: GameId(4), picks: vec![
            CubeSet { red: 3, green: 1, blue: 6 }, 
            CubeSet { red: 6, green: 3, blue: 0 }, 
            CubeSet { red: 14, green: 3, blue: 15 }]
        })]
    #[case(
        "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green", 
        Game { id: GameId(5), picks: vec![
            CubeSet { red: 6, green: 3, blue: 1 }, 
            CubeSet { red: 1, green: 2, blue: 2 }]
        })]
    fn test_cases(#[case] input: &str, #[case] expected: Game) {
        assert_eq!(Ok(expected), Game::from_str(input));
    }

}
