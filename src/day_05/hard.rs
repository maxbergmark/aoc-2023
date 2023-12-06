use std::str::FromStr;

use crate::common::error::AocError;

use super::common::{Almanac, Mapping};

type Range = std::ops::Range<i64>;

struct RangeMapping {
    range: Range,
    offset: i64,
}
#[derive(Debug)]
enum MaybeMapped {
    Unmapped(Range),
    Mapped(Range),
}

impl MaybeMapped {
    fn reset(self) -> Self {
        match self {
            MaybeMapped::Unmapped(_) => self,
            MaybeMapped::Mapped(range) => MaybeMapped::Unmapped(range),
        }
    }
    fn into_range(self) -> Range {
        match self {
            MaybeMapped::Unmapped(range) => range,
            MaybeMapped::Mapped(range) => range,
        }
    }
}

fn split_range(range: Range, mapping: &RangeMapping) -> Vec<MaybeMapped> {
    if range.end <= mapping.range.start || mapping.range.end <= range.start { // mutually exclusive
        vec![MaybeMapped::Unmapped(range)]
    } else if range.start < mapping.range.start && range.end > mapping.range.end { // range overlaps completely
        vec![
            // split to three
            MaybeMapped::Unmapped(range.start..mapping.range.start),
            MaybeMapped::Mapped(mapping.range.start+mapping.offset..mapping.range.end+mapping.offset),
            MaybeMapped::Unmapped(mapping.range.end..range.end),
        ]
    } else if range.start < mapping.range.start && mapping.range.start < range.end { // overlap left
        vec![
            // split two
            MaybeMapped::Unmapped(range.start..mapping.range.start),
            MaybeMapped::Mapped(mapping.range.start+mapping.offset..range.end+mapping.offset),
        ]
    } else if range.start < mapping.range.end && mapping.range.end < range.end { // overlap right
        vec![
            // split two
            MaybeMapped::Mapped(range.start+mapping.offset..mapping.range.end+mapping.offset),
            MaybeMapped::Unmapped(mapping.range.end..range.end),
        ]
    } else { // fully contained
        vec![
            MaybeMapped::Mapped(range.start+mapping.offset..range.end+mapping.offset)
        ]
    }
}

fn maybe_map(ranges: Vec<MaybeMapped>, mapping: RangeMapping) -> Vec<MaybeMapped> {
    ranges.into_iter()
        .flat_map(|range| {
            match range {
                MaybeMapped::Unmapped(r) => split_range(r, &mapping),
                MaybeMapped::Mapped(_) => vec![range],
            }
        })
        .collect()
}

fn to_rangemapping(mapping: Mapping) -> RangeMapping {
    RangeMapping { 
        range: mapping.from..mapping.from + mapping.length,
        offset: mapping.to - mapping.from
    }
}

fn chunk_to_range(chunk: &[i64]) -> Result<Range, AocError> {
    match chunk {
        [start, length] => Ok(*start..*start + *length),
        _ => Err(AocError::ParseError),
    }
}

fn to_ranges(seeds: &[i64]) -> Result<Vec<Range>, AocError> {
    let chunks: Vec<&[i64]> = seeds.chunks(2).collect();
    let ranges: Result<Vec<Range>, AocError> = chunks.into_iter()
        .map(chunk_to_range)
        .collect();

    ranges
}

fn map_all(mut ranges: Vec<MaybeMapped>, mappings: Vec<RangeMapping>) -> Vec<MaybeMapped> {
    for mapping in mappings {
        ranges = maybe_map(ranges, mapping);
    }
    ranges
}

fn map_and_reset(mut ranges: Vec<MaybeMapped>, mappings: Vec<RangeMapping>) -> Vec<MaybeMapped> {
    ranges = map_all(ranges, mappings);
    ranges.into_iter().map(|r| r.reset()).collect()
}

trait ToRangeMapping {
    fn to_rangemapping(self) -> Vec<RangeMapping>;
}

impl ToRangeMapping for Vec<Mapping> {
    fn to_rangemapping(self) -> Vec<RangeMapping> {
        self.into_iter().map(to_rangemapping).collect()
    }
}

fn map_seed_ranges(almanac: Almanac, ranges: Vec<Range>) -> Vec<Range> {

    let ranges = ranges.into_iter()
        .map(MaybeMapped::Unmapped).collect();

    let ranges = almanac.into_maps().into_iter()
        .map(Vec::<Mapping>::to_rangemapping)
        .fold(ranges, map_and_reset);

    ranges.into_iter()
        .map(MaybeMapped::into_range)
        .collect()
}

fn solve_almanac(almanac: Almanac) -> Result<i64, AocError> {
    let ranges = to_ranges(&almanac.seeds)?;
    let locations = map_seed_ranges(almanac, ranges);
    locations.into_iter()
        .map(|range| range.start)
        .min()
        .ok_or(AocError::ParseError)

}

fn solve_file(filename: &str) -> Result<i64, AocError> {
    let s = std::fs::read_to_string(filename).map_err(|_| AocError::FileNotFound)?;
    let almanac = Almanac::from_str(s.as_str())?;
    solve_almanac(almanac)
}

#[allow(unused)]
pub fn solve() -> Result<i64, AocError> {
    solve_file("input/day_05/puzzle.txt")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_testcase() {
        assert_eq!(Ok(46), solve_file("input/day_05/easy_test.txt"));
    }

    #[test]
    fn test_solve() {
        assert_eq!(Ok(34039469), solve())
    }

    #[test]
    fn test_single_number() {
        let mut s = std::fs::read_to_string("input/day_05/easy_test.txt").unwrap();
        s = s.replace("79 14 55 13", "82 1");
        let almanac = Almanac::from_str(s.as_str()).unwrap();
        let ranges = to_ranges(&almanac.seeds).unwrap();
        let locations = map_seed_ranges(almanac, ranges);
        let min = locations.into_iter()
            .min_by(|a, b| a.start.cmp(&b.start));
        assert_eq!(46, min.unwrap().start);
    }
}