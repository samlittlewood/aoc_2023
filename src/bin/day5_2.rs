use std::fs;
//use itertools::Itertools;

#[derive(Debug)]
struct Map {
    destination_start: usize,
    source_start: usize,
    range_length: usize
}

#[derive(Debug,PartialEq,Clone)]
struct Range {
    start: usize,
    length: usize
}

impl Map {
    fn from_input<I>(input: & mut I) -> Vec<Map> where I: Iterator<Item = String> {
        let mut m = vec! [];

        while let Some(l) = input.next() {
            let n = l.split(' ').map(|s| s.parse::<usize>()).collect::<Result<Vec<_>,_>>();
            if let Ok(nums) = n {
                m.push(Map { destination_start: nums[0], source_start: nums[1], range_length: nums[2]})
            } else {
                break;
            }
        }

        m
    }

    fn convert(&self, seeds:&Range, mapped: &mut Vec<Range>, unmapped: &mut Vec<Range>) {
        let seeds_end = seeds.start + seeds.length;
        let source_end = self.source_start + self.range_length;

        // part of seeds range before map range
        if seeds.start < self.source_start {
            let start = seeds.start;
            let length = usize::min(seeds.length, self.source_start-seeds.start);
            unmapped.push(Range { start, length });
        }

        // part of seed range covered by map range
        if seeds_end > self.source_start && seeds.start < source_end {
            let start = usize::max(seeds.start, self.source_start);
            let length = usize::min(source_end - start, seeds_end - start);
            mapped.push(Range { start:(start - self.source_start) + self.destination_start, length });
        }

        // part of seed range after map range
        if seeds_end > source_end {
            let start = usize::max(seeds.start, source_end);
            let length = usize::min(seeds.length, seeds_end - source_end);
            unmapped.push(Range { start, length });
        }
    }
}

fn convert_ranges(ranges : &Vec<Map>, seeds: &Vec<Range>) -> Vec<Range> {
    let mut output = vec![];
    let mut input = seeds.clone();

    for m in ranges {
         let mut unmapped = vec![];
           for s in &input {
               m.convert(s, &mut output, &mut unmapped);
        }

        input.clear();
        input.append(&mut unmapped);
    }

    output.append(&mut input);
    output.sort_by_key(|r| r.start);
    return output;
}

fn main() {
    println!("Hello, Day 5 2!");

    // For each card, figure wins
    let input = fs::read_to_string("inputs/day5").unwrap();
    let mut input_iter = input.lines().map(|s| s.to_string());
    let seeds = input_iter.next().unwrap()[7..].split(' ').
        map(|s| s.parse::<usize>().unwrap()).collect::<Vec<_>>();

    let mut seed_to_soil = vec! [];
    let mut fertilizer_to_water = vec! [];
    let mut water_to_light = vec! [];
    let mut light_to_temperature = vec! [];
    let mut temperature_to_humidity = vec! [];
    let mut humidity_to_location = vec! [];
    let mut soil_to_fertilizer = vec! [];

    while let Some(r) = input_iter.next() {
        match r.as_str() {
                "seed-to-soil map:" => seed_to_soil = Map::from_input(&mut input_iter),
                "soil-to-fertilizer map:" => soil_to_fertilizer = Map::from_input(&mut input_iter),
                "fertilizer-to-water map:" => fertilizer_to_water = Map::from_input(&mut input_iter),
                "water-to-light map:" => water_to_light = Map::from_input(&mut input_iter),
                "light-to-temperature map:" => light_to_temperature = Map::from_input(&mut input_iter),
                "temperature-to-humidity map:" => temperature_to_humidity = Map::from_input(&mut input_iter),
                "humidity-to-location map:" => humidity_to_location = Map::from_input(&mut input_iter),
                _ => (),
        }
    }

    let mut seed_ranges = vec![];
    let mut seeds_iter = seeds.into_iter();
    loop {
        if let Some(start) = seeds_iter.next() {
            if let Some(length) = seeds_iter.next() {
                seed_ranges.push(Range{start, length});
                continue;
            }
        }
        break;
    }
    println!("Seeds: {seed_ranges:?}");

    let soil = convert_ranges(&seed_to_soil, &seed_ranges);
    let fertilizer = convert_ranges(&soil_to_fertilizer, &soil);
    let water = convert_ranges(&fertilizer_to_water, &fertilizer);
    let light = convert_ranges(&water_to_light, &water);
    let temperature = convert_ranges(&light_to_temperature, &light);
    let humidity = convert_ranges(&temperature_to_humidity, &temperature);
    let location = convert_ranges(&humidity_to_location, &humidity);

    println!("Location: {}", location[0].start);
}

#[test]
fn test1() {
    let map = vec![
        Map { source_start:100, destination_start:1000, range_length: 10},
    ];

    assert_eq!(convert_ranges(&map, &vec![Range { start: 101, length: 5}]), vec![Range { start: 1001, length: 5 } ]);
    assert_eq!(convert_ranges(&map, &vec![Range { start: 90, length: 5}]), vec![Range { start: 90, length: 5 } ]);
    assert_eq!(convert_ranges(&map, &vec![Range { start: 90, length: 10}]), vec![Range { start: 90, length: 10 } ]);
    assert_eq!(convert_ranges(&map, &vec![Range { start: 90, length: 11}]), vec![Range { start: 1000, length: 1 }, Range { start: 90, length: 10 }]);
    assert_eq!(convert_ranges(&map, &vec![Range { start: 109, length: 2}]), vec![Range { start: 1009, length: 1 }, Range { start: 110, length: 1 }]);
    assert_eq!(convert_ranges(&map, &vec![Range { start: 90, length: 32}]), vec![Range { start: 1000, length: 10 }, Range { start: 90, length: 10 }, Range { start: 110, length: 12 }]);
}

#[test]
fn test2() {
    let map = vec![
        Map { source_start:100, destination_start:1000, range_length: 10},
        Map { source_start:120, destination_start:2000, range_length: 32},
    ];

    assert_eq!(convert_ranges(&map, &vec![Range { start: 101, length: 5}]), vec![Range { start: 1001, length: 5 } ]);
    assert_eq!(convert_ranges(&map, &vec![Range { start: 121, length: 7}]), vec![Range { start: 2001, length: 7 } ]);
    assert_eq!(convert_ranges(&map, &vec![Range { start: 90, length: 32} ]), vec![Range { start: 1000, length: 10 }, Range { start: 2000, length: 2 }, Range { start: 90, length: 10 }, Range { start: 110, length: 10 }]);
    assert_eq!(convert_ranges(&map, &vec![Range { start: 90, length: 82} ]), vec![Range { start: 1000, length: 10 }, Range { start: 2000, length: 32 }, Range { start: 90, length: 10 }, Range { start: 110, length: 10 }, Range { start: 152, length: 20 }]);
    assert_eq!(convert_ranges(&map, &vec![Range { start: 101, length: 7}, Range { start: 121, length: 7}]), vec![Range { start: 2001, length: 7 } ]);
}
