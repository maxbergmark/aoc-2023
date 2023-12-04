use std::{fs::File, io::{BufReader, BufRead}};

use super::error::AocError;


pub fn file_to_lines(filename: &str) -> Result<std::io::Lines<BufReader<File>>, AocError> {
    let file = File::open(filename)
        .map_err(|_| AocError::FileNotFound)?;
    let reader = BufReader::new(file);
    Ok(reader.lines())
}

pub fn to_parse_error<T, E>(res: Result<T, E>) -> Result<T, AocError> {
    res.map_err(|_| AocError::ParseError)
}