// very shitty algorithm lol

use std::{
    cmp::Ordering,
    collections::HashMap,
    default,
    num::{self, ParseIntError},
    ops::Range,
};

use nom::{
    branch::alt,
    bytes::complete::{tag, take_until, take_while},
    character::{
        complete::{
            alpha1, anychar, crlf, digit0, digit1, line_ending, multispace0, newline, space1,
        },
        is_alphabetic,
    },
    combinator::eof,
    error,
    sequence::{tuple, Tuple},
    IResult,
};

mod test;

fn main() {
    let file = std::env::args().nth(1).unwrap();

    let file_content = std::fs::read_to_string(file).unwrap();

    println!("Part 1: {}", process_part_one(&file_content));
    println!("Part 2: {}", process_part_two(&file_content));
}

#[derive(Debug, Clone)]
struct MapRange {
    source: Range<usize>,
    destination: Range<usize>,
    // length: usize,
}

impl From<&str> for MapRange {
    fn from(value: &str) -> Self {
        let splitted: Vec<usize> = value
            .split_whitespace()
            .map(|v| v.trim().parse::<usize>().unwrap())
            .collect();

        let dest_start = splitted.get(0).unwrap().clone();
        let source_start = splitted.get(1).unwrap().clone();
        let length = splitted.get(2).unwrap();

        MapRange {
            destination: dest_start..dest_start + length,
            source: source_start..source_start + length,
        }
    }
}

impl PartialEq for MapRange {
    fn eq(&self, other: &Self) -> bool {
        self.source == other.source && self.destination == other.destination
    }
}

impl PartialOrd for MapRange {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.clone().source.partial_cmp(other.clone().source) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.clone()
            .destination
            .partial_cmp(other.clone().destination)
    }
}

impl Ord for MapRange {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.clone().source.min() < other.clone().source.min() {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    }
}

impl Eq for MapRange {}

impl MapRange {
    fn get_mapped(&self, value: usize) -> Option<usize> {
        if self.source.contains(&value) {
            let min_value = self.source.clone().into_iter().nth(0).unwrap();
            let mut zipped = self.source.clone().zip(self.destination.clone());

            let index: isize = min_value as isize - value as isize;
            return Some(zipped.nth(index.abs() as usize).unwrap().1);
        }

        None
    }
}

#[derive(Debug, Clone)]
struct ItemToItemRange {
    name: String,
    maps: Vec<MapRange>,
}

impl From<&str> for ItemToItemRange {
    fn from(value: &str) -> Self {
        let lines: Vec<&str> = value
            .lines()
            .map(|l| l.trim())
            .filter(|l| !l.is_empty())
            .collect();

        let name = lines.get(0).unwrap().replace(" map:", "");
        let mut maps: Vec<MapRange> = lines[1..].iter().map(|l| l.clone().into()).collect();
        maps.sort();

        ItemToItemRange { name, maps }
    }
}

impl PartialEq for ItemToItemRange {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.maps == other.maps
    }
}

impl ItemToItemRange {
    fn get_mapped(&self, value: usize) -> usize {
        for map in self.maps.clone() {
            if let Some(mapped_value) = map.get_mapped(value) {
                return mapped_value;
            }
        }

        value
    }
}

fn process_input(input: Vec<&str>) -> HashMap<String, ItemToItemRange> {
    let mut item_maps: HashMap<String, ItemToItemRange> = HashMap::default();

    for comp in input {
        let item_range: ItemToItemRange = comp.into();

        item_maps.insert(item_range.name.clone(), item_range);
    }

    item_maps
}

#[derive(Debug, Clone)]
struct SeedPart {
    seed: usize,
    soil: usize,
    fertilizer: usize,
    water: usize,
    light: usize,
    temperature: usize,
    humidity: usize,
    location: usize,
}

impl PartialEq for SeedPart {
    fn eq(&self, other: &Self) -> bool {
        self.seed == other.seed
            && self.soil == other.soil
            && self.fertilizer == other.fertilizer
            && self.water == other.water
            && self.light == other.light
            && self.temperature == other.temperature
            && self.humidity == other.humidity
            && self.location == other.location
    }
}

fn get_seed_parts(seed: usize, maps: HashMap<String, ItemToItemRange>) -> SeedPart {
    let soil = maps.get("seed-to-soil").unwrap().get_mapped(seed);
    let fertilizer = maps.get("soil-to-fertilizer").unwrap().get_mapped(soil);
    let water = maps
        .get("fertilizer-to-water")
        .unwrap()
        .get_mapped(fertilizer);
    let light = maps.get("water-to-light").unwrap().get_mapped(water);
    let temperature = maps.get("light-to-temperature").unwrap().get_mapped(light);
    let humidity = maps
        .get("temperature-to-humidity")
        .unwrap()
        .get_mapped(temperature);
    let location = maps
        .get("humidity-to-location")
        .unwrap()
        .get_mapped(humidity);

    SeedPart {
        seed,
        soil,
        fertilizer,
        water,
        light,
        temperature,
        humidity,
        location,
    }
}

fn process_part_one(input: &str) -> usize {
    let mut line = input.split("\r\n\r");
    let maps = process_input(line.clone().collect());
    line.nth(0)
        .unwrap()
        .replace("seeds: ", "")
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .map(|s| get_seed_parts(s, maps.clone()))
        .map(|p| {
            // dbg!(&p);
            p.location
        })
        .min()
        .unwrap()
}

// redid everything :facepalm:

#[derive(Debug, Clone, Copy)]
struct OptRange {
    start: usize,
    end: usize,
}

impl PartialOrd for OptRange {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.start.partial_cmp(&other.start) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.end.partial_cmp(&other.end)
    }
}

impl PartialEq for OptRange {
    fn eq(&self, other: &Self) -> bool {
        self.start == other.start && self.end == other.end
    }
}

impl Ord for OptRange {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.start > other.start {
            Ordering::Greater
        } else if self.start == other.start {
            Ordering::Equal
        } else {
            Ordering::Less
        }
    }
}

impl Eq for OptRange {}

#[derive(Debug, Clone, Copy)]
struct OptRangeMap {
    source: OptRange,
    destination: OptRange,
    length: usize,
}

impl PartialEq for OptRangeMap {
    fn eq(&self, other: &Self) -> bool {
        self.source == other.source && self.destination == other.destination
    }
}

impl PartialOrd for OptRangeMap {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.source.partial_cmp(&other.source) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.destination.partial_cmp(&other.destination)
    }
}

impl Eq for OptRangeMap {}
impl Ord for OptRangeMap {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.source > other.source {
            Ordering::Greater
        } else if self.source == other.source {
            Ordering::Equal
        } else {
            Ordering::Less
        }
    }
}

impl From<&str> for OptRangeMap {
    fn from(value: &str) -> Self {
        let mut splitted = value.split(' ');

        let dest: usize = splitted.next().unwrap().parse().unwrap();
        let source: usize = splitted.next().unwrap().parse().unwrap();
        let length: usize = splitted.next().unwrap().parse().unwrap();

        OptRangeMap {
            destination: OptRange {
                start: dest,
                end: dest + length,
            },
            source: OptRange {
                start: source,
                end: source + length,
            },
            length,
        }
    }
}

#[derive(Debug, Clone)]
struct OptElemMapping {
    name: String,
    maps: Vec<OptRangeMap>,
}

fn take_until_double_line_ending(input: &str) -> IResult<&str, &str> {
    alt((take_until("\r\n\r"), take_until("\n\n")))(input.trim())
}

fn get_seeds(input: &str) -> Vec<usize> {
    input
        .trim()
        .replace("seeds: ", "")
        .split(' ')
        .map(|s| s.parse().unwrap())
        .collect()
}

fn get_map(input: &str) -> OptElemMapping {
    let (input, map_name) = alt((
        take_until::<&str, &str, nom::error::Error<&str>>("\n"),
        take_until::<&str, &str, nom::error::Error<&str>>("\r\n"),
    ))(input)
    .unwrap();

    let mut maps: Vec<OptRangeMap> = Vec::new();
    for line in input.trim().lines() {
        maps.push(line.into());
    }

    maps.sort();

    OptElemMapping {
        maps,
        name: map_name.replace("map:", "").trim().to_string(),
    }
}

impl OptRangeMap {
    fn get_mapped(&self, number: usize) -> Option<usize> {
        if number < self.source.end && number >= self.source.start {
            return Some(
                (self.source.start as isize - number as isize).abs() as usize
                    + self.destination.start,
            );
        }

        None
    }
}

impl OptElemMapping {
    fn get_mapped(&self, index: usize) -> usize {
        for map in self.maps.clone() {
            if let Some(v) = map.get_mapped(index) {
                return v;
            }
        }

        index
    }
}

#[derive(Debug)]
struct Mapper {
    seeds: Vec<usize>,
    seed_to_soil: OptElemMapping,
    soil_to_fertilizer: OptElemMapping,
    fertilizer_to_water: OptElemMapping,
    water_to_light: OptElemMapping,
    light_to_temperature: OptElemMapping,
    temperature_to_humidity: OptElemMapping,
    humidity_to_location: OptElemMapping,
}

fn parse_input(input: &str) -> Mapper {
    let (input, seeds) = take_until_double_line_ending(input).unwrap();
    let (input, seed_to_soil) = take_until_double_line_ending(input).unwrap();
    let (input, soil_to_fertilizer) = take_until_double_line_ending(input).unwrap();
    let (input, fertilizer_to_water) = take_until_double_line_ending(input).unwrap();
    let (input, water_to_light) = take_until_double_line_ending(input).unwrap();
    let (input, light_to_temperature) = take_until_double_line_ending(input).unwrap();
    let (humidity_to_location, temperature_to_humidity) =
        take_until_double_line_ending(input).unwrap();

    let humidity_to_location = humidity_to_location.trim();

    let seeds = get_seeds(seeds);
    let seed_to_soil = get_map(seed_to_soil);
    let soil_to_fertilizer = get_map(soil_to_fertilizer);
    let fertilizer_to_water = get_map(fertilizer_to_water);
    let water_to_light = get_map(water_to_light);
    let light_to_temperature = get_map(light_to_temperature);
    let temperature_to_humidity = get_map(temperature_to_humidity);
    let humidity_to_location = get_map(humidity_to_location);

    Mapper {
        seeds,
        seed_to_soil,
        soil_to_fertilizer,
        fertilizer_to_water,
        water_to_light,
        light_to_temperature,
        temperature_to_humidity,
        humidity_to_location,
    }
}

fn get_seed_range(seeds: Vec<usize>) -> Vec<Range<usize>> {
    let mut seeds = seeds.iter();
    let mut should_continue = true;
    let mut buf: Option<usize> = None;
    let mut res_seeds: Vec<Range<usize>> = Vec::new();
    while should_continue {
        if let Some(b) = buf {
            let curr = seeds.next();

            if let Some(curr) = curr {
                res_seeds.push(b..*curr);
                buf = None;
            } else {
                should_continue = false;
            }
        } else {
        }
    }

    res_seeds
}

fn process_part_two(input: &str) -> usize {
    let mapper = parse_input(input);

    let seeds = get_seed_range(mapper.seeds);
    let locations: Vec<usize> = Vec::new();
    // for seed_range in seeds {
    //     for seed in seed_range {
    //         dbg!("what");
    //     }
    // }

    0
}
