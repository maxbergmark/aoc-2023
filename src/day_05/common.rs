use std::str::FromStr;

use crate::common::{error::AocError, functions::to_parse_error};


#[derive(Debug)]
pub struct Mapping {
    pub from: i64,
    pub to: i64,
    pub length: i64,
}

impl Mapping {
    pub fn map(&self, n: i64) -> Option<i64> {
        let start = self.from;
        let end = start + self.length;
        if (start..end).contains(&n) {
            Some(n + self.to - start)
        } else {
            None
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct SeedRange {
    pub from: i64,
    pub length: i64,
}

#[derive(Debug)]
pub struct Almanac {
    pub seeds: Vec<i64>,
    pub seed_to_soil_map: Vec<Mapping>,
    pub soil_to_fertilizer_map: Vec<Mapping>,
    pub fertilizer_to_water_map: Vec<Mapping>,
    pub water_to_light_map: Vec<Mapping>,
    pub light_to_temperature_map: Vec<Mapping>,
    pub temperature_to_humidity_map: Vec<Mapping>,
    pub humidity_to_location_map: Vec<Mapping>,
}

impl Almanac {

    pub fn into_maps(self) -> Vec<Vec<Mapping>> {
        vec![
            self.seed_to_soil_map,
            self.soil_to_fertilizer_map,
            self.fertilizer_to_water_map,
            self.water_to_light_map,
            self.light_to_temperature_map,
            self.temperature_to_humidity_map,
            self.humidity_to_location_map,
        ]
    }

    pub fn seed_to_location(&self, seed: i64) -> i64 {
        let soil = Self::map_to_next(seed, &self.seed_to_soil_map);
        let fert = Self::map_to_next(soil, &self.soil_to_fertilizer_map);
        let water = Self::map_to_next(fert, &self.fertilizer_to_water_map);
        let light = Self::map_to_next(water, &self.water_to_light_map);
        let temp = Self::map_to_next(light, &self.light_to_temperature_map);
        let humid = Self::map_to_next(temp, &self.temperature_to_humidity_map);
        Self::map_to_next(humid, &self.humidity_to_location_map)
    }

    fn map_to_next(n: i64, mappings: &[Mapping]) -> i64 {
        mappings.iter()
            .find_map(|mapping| mapping.map(n))
            .unwrap_or(n)
    }
}

fn parse_seeds(seeds: &str) -> Result<Vec<i64>, AocError> {
    let seeds = seeds.split(": ").last().ok_or(AocError::ParseError)?;
    seeds.split_ascii_whitespace()
        .map(i64::from_str)
        .map(to_parse_error)
        .collect()
}

fn parse_map(map: &str) -> Result<Vec<Mapping>, AocError> {
    let res: Result<Vec<Mapping>, AocError> = map.lines().skip(1)
        .map(Mapping::from_str)
        .collect();
    let mut v = res?;
    v.sort_by(|a, b| a.from.cmp(&b.from));
    Ok(v)
}

impl FromStr for Almanac {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split("\n\n").collect();
        match parts.as_slice() {
            [seeds, seed_to_soil, soil_to_fertilizer, fertilizer_to_water, water_to_light, 
            light_to_temperature, temperature_to_humidity, humidity_to_location] => {
                // println!("{parts:?}");
                Ok(Almanac {
                    seeds: parse_seeds(seeds)?,
                    seed_to_soil_map: parse_map(seed_to_soil)?,
                    soil_to_fertilizer_map: parse_map(soil_to_fertilizer)?,
                    fertilizer_to_water_map: parse_map(fertilizer_to_water)?,
                    water_to_light_map: parse_map(water_to_light)?,
                    light_to_temperature_map: parse_map(light_to_temperature)?,
                    temperature_to_humidity_map: parse_map(temperature_to_humidity)?,
                    humidity_to_location_map: parse_map(humidity_to_location)?, 
                })
            },
            _ => Err(AocError::ParseError)
        }
    }
}

impl FromStr for Mapping {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_ascii_whitespace().collect();
        match parts.as_slice() {
            [to, from, length] => {
                Ok(Mapping {
                    from: i64::from_str(from).map_err(|_| AocError::ParseError)?,
                    to: i64::from_str(to).map_err(|_| AocError::ParseError)?,
                    length: i64::from_str(length).map_err(|_| AocError::ParseError)?,
                })
            },
            _ => Err(AocError::ParseError)
        }
    }
}