use std::fs;

#[derive(Debug)]
struct Map {
    destination_start: usize,
    source_start: usize,
    range_length: usize
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

    fn convert(&self, seed:usize) -> Option<usize> {
        if seed >= self.source_start && seed < self.source_start + self.range_length {
            Some(self.destination_start + (seed - self.source_start))
        } else {
            None
        }
    }
}

fn convert_ranges(ranges : &Vec<Map>, value: usize) -> usize {
    for range in ranges {
        if let Some(r) = range.convert(value) {
            return r;
        }
    }

    return value;
}

fn main() {
    println!("Hello, Day 5 1!");

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

    let mut nearest = usize::MAX;
    for seed in seeds {
        let soil = convert_ranges(&seed_to_soil, seed);
        let fertilizer = convert_ranges(&soil_to_fertilizer, soil);
        let water = convert_ranges(&fertilizer_to_water, fertilizer);
        let light = convert_ranges(&water_to_light, water);
        let temperature = convert_ranges(&light_to_temperature, light);
        let humidity = convert_ranges(&temperature_to_humidity, temperature);
        let location = convert_ranges(&humidity_to_location, humidity);

 //       println!("seed:{seed} soil:{soil}, fertilizer:{fertilizer}, water:{water} light:{light}, temperature:{temperature}, h:{humidity}, location:{location}");
        if location < nearest {
            nearest = location
        }
    }
    println!("Nearest {nearest}");
}
