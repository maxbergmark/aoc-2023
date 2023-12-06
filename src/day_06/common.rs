use std::str::FromStr;

use crate::common::{error::AocError, traits::ToAocError};



#[derive(Debug)]
pub struct Race {
    pub time: i64,
    pub distance: i64,
}

pub fn num_possibilities(race: Race) -> i64 {
    let p = -race.time as f64;
    let q = race.distance as f64;
    let x_0 = -p/2.0 + f64::sqrt(p*p/4.0 - q);
    let x_1 = -p/2.0 - f64::sqrt(p*p/4.0 - q);

    (x_0.ceil() - x_1.floor() - 1.0) as i64
}


fn to_vec(line: &str) -> Result<Vec<i64>, AocError> {
    line.split_ascii_whitespace()
        .skip(1)
        .map(|s| i64::from_str(s).to_aoc_error(AocError::ParseError))
        .collect()
}

pub fn parse(filename: &str) -> Result<Vec<Race>, AocError> {
    let s = std::fs::read_to_string(filename)
        .to_aoc_error(AocError::FileNotFound)?;
    let mut lines = s.lines();
    let time_str = lines.next().ok_or(AocError::ParseError)?;
    let distance_str = lines.next().ok_or(AocError::ParseError)?;

    let times = to_vec(time_str)?;
    let distances = to_vec(distance_str)?;

    Ok(times.into_iter().zip(distances)
        .map(|(time, distance)| Race { time, distance })
        .collect())
}

pub fn parse_as_single(filename: &str) -> Result<Race, AocError> {
    let s = std::fs::read_to_string(filename)
    .to_aoc_error(AocError::FileNotFound)?;
    let s = s.replace(' ', "");
    let mut lines = s.lines();
    let time_str = lines.next().ok_or(AocError::ParseError)?;
    let distance_str = lines.next().ok_or(AocError::ParseError)?;

    let time = time_str.split(':').last().ok_or(AocError::ParseError)
        .map(|s| i64::from_str(s).to_aoc_error(AocError::ParseError))??;
    let distance = distance_str.split(':').last().ok_or(AocError::ParseError)
        .map(|s| i64::from_str(s).to_aoc_error(AocError::ParseError))??;

    Ok(Race {
        time,
        distance,
    })

}