use aoc2023::read_lines;

struct Mapping {
    source_start: i64,
    source_end: i64,
    offset: i64,
}

impl Mapping {
    fn map(&self, key: &i64) -> Option<i64> {
        if key < &self.source_start || key > &self.source_end {
            None
        } else {
            Some(key + self.offset)
        }
    }
}

struct ElfAlmanacMap {
    mappings: Vec<Mapping>,
}

impl ElfAlmanacMap {
    fn get_mapping(&self, key: &i64) -> i64 {
        for mapping in &self.mappings {
            if let Some(value) = mapping.map(key) {
                return value;
            }
        }
        *key
    }
}

struct ElfAlmanac {
    seed_to_soil: ElfAlmanacMap,
    soil_to_fertilizer: ElfAlmanacMap,
    fertilizer_to_water: ElfAlmanacMap,
    water_to_light: ElfAlmanacMap,
    light_to_temperature: ElfAlmanacMap,
    temperature_to_humidity: ElfAlmanacMap,
    humidity_to_location: ElfAlmanacMap,
}

fn parse_mappings(iter: &mut dyn Iterator<Item = &String>) -> Vec<Mapping> {
    let mut mappings: Vec<Mapping> = vec![];
    for line in iter {
        if line.trim().is_empty() {
            return mappings;
        }
        let mut s = line.split_whitespace();
        let destination_start = s.next().unwrap().parse::<i64>().unwrap();
        let source_start = s.next().unwrap().parse::<i64>().unwrap();
        let offset = destination_start - source_start;
        let len = s.next().unwrap().parse::<i64>().unwrap();
        let mapping = Mapping {
            source_start,
            source_end: source_start + len - 1,
            offset,
        };
        mappings.push(mapping);
    }
    mappings
}

fn parse(iter: &mut dyn Iterator<Item = &String>) -> ElfAlmanac {
    let mut maps: Vec<ElfAlmanacMap> = vec![];
    while let Some(line) = iter.next() {
        // if a line ends with a colon, a map is starting on the next line
        if line.ends_with(':') {
            // consume the iterator until the next newline, parsing the mappings
            let mappings = parse_mappings(iter);
            maps.push(ElfAlmanacMap { mappings });
        }
    }

    let mut maps_iter = maps.into_iter();

    ElfAlmanac {
        seed_to_soil: maps_iter.next().unwrap(),
        soil_to_fertilizer: maps_iter.next().unwrap(),
        fertilizer_to_water: maps_iter.next().unwrap(),
        water_to_light: maps_iter.next().unwrap(),
        light_to_temperature: maps_iter.next().unwrap(),
        temperature_to_humidity: maps_iter.next().unwrap(),
        humidity_to_location: maps_iter.next().unwrap(),
    }
}

fn seeds_1(line: &str) -> Vec<i64> {
    let (_, _seeds) = line.split_once(": ").unwrap();
    _seeds
        .split_whitespace()
        .map(|seed| seed.parse().unwrap())
        .collect::<Vec<i64>>()
}

fn seeds_2(line: &str) -> Vec<i64> {
    let (_, _seeds) = line.split_once(": ").unwrap();
    let mut v = vec![];
    let mut iter = _seeds.split_whitespace();
    while let Some(range_start) = iter.next() {
        let parsed_start: i64 = range_start.parse().unwrap();
        let parsed_len: i64 = iter.next().unwrap().parse().unwrap();
        for x in parsed_start..parsed_start + parsed_len {
            v.push(x);
        }
    }
    v
}

fn calculate(seeds: &[i64], almanac: &mut ElfAlmanac) {
    let min = seeds
        .iter()
        .map(|seed| {
            let soil = almanac.seed_to_soil.get_mapping(seed);
            let fertilizer = almanac.soil_to_fertilizer.get_mapping(&soil);
            let water = almanac.fertilizer_to_water.get_mapping(&fertilizer);
            let light = almanac.water_to_light.get_mapping(&water);
            let temperature = almanac.light_to_temperature.get_mapping(&light);
            let humidity = almanac.temperature_to_humidity.get_mapping(&temperature);
            
            almanac.humidity_to_location.get_mapping(&humidity)
        })
        .min()
        .unwrap();

    println!("{}", min);
}

fn main() {
    let lines = read_lines("inputs/day5");
    let mut iter = lines.iter();
    let _seeds = iter.next().unwrap();

    let s1 = seeds_1(_seeds);
    let s2 = seeds_2(_seeds);

    let mut almanac = parse(&mut iter);
    calculate(&s1, &mut almanac);
    calculate(&s2, &mut almanac);
}
