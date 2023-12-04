use std::str::FromStr;

use crate::common::error::AocError;



#[derive(Debug, PartialEq)]
pub struct GameId(pub i32);

#[derive(Debug, PartialEq)]
pub struct Game {
    pub id: GameId,
    pub picks: Vec<CubeSet>,
}

#[derive(Debug, Default, PartialEq)]
pub struct CubeSet {
    pub red: i32,
    pub green: i32,
    pub blue: i32,
}

impl FromStr for Game {
    type Err = AocError;

    // Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(": ").collect();
        match parts.as_slice() {
            [header, rest] => {
                let cube_sets: Result<Vec<CubeSet>, AocError> = rest.split("; ")
                    .map(CubeSet::from_str)
                    .collect();
                Ok(Game {
                    id: GameId::from_str(header)?,
                    picks: cube_sets?,
                })
            },
            _ => Err(AocError::ParseError),
        }
    }
}

impl FromStr for GameId {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let id_part = s.split(' ').last().ok_or(AocError::ParseError)?;
        let id = i32::from_str(id_part).map_err(|_| AocError::ParseError)?;
        Ok(GameId(id))
    }
}

impl FromStr for CubeSet {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(", ");
        let mut cube_set = CubeSet::default();
        for part in parts {
            let set: Vec<&str> = part.split(' ').collect();
            match set.as_slice() {
                [num, color] => {
                    let num = i32::from_str(num).map_err(|_| AocError::ParseError)?;
                    match *color {
                        "red" => cube_set.red = num,
                        "green" => cube_set.green = num,
                        "blue" => cube_set.blue = num,
                        _ => Err(AocError::ParseError)?
                    }
                },
                _ => Err(AocError::ParseError)?
            }
        }
        Ok(cube_set)
    }
}
