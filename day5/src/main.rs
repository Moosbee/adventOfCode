use std::fs;

#[derive(Debug)]
pub struct Mapping {
    dest_start: i64,
    source_start: i64,
    range_length: i64,
}

#[derive(Debug)]
pub struct Map {
    mappings: Vec<Mapping>,
}

impl Map {
    pub fn to_map(input: &str, prefix: &str) -> Map {
        let mappings: Vec<Mapping> = input
            .trim()
            .strip_prefix(prefix)
            .unwrap_or_default()
            .split("\n")
            .filter_map(|f| {
                let maps: Vec<i64> = f
                    .split(" ")
                    .filter_map(|s| s.parse().ok())
                    .collect();
                if maps.len() == 3 {
                    Some(Mapping {
                        dest_start: maps[0],
                        source_start: maps[1],
                        range_length: maps[2],
                    })
                } else {
                    None
                }
            })
            .collect();

        Map {
            mappings,
        }
    }

    pub fn get_map_value(&self, source: i64) -> i64 {
        for map_ing in &self.mappings {
            if
                map_ing.source_start < source &&
                map_ing.source_start + map_ing.range_length > source
            {
                let source_diff = source - map_ing.source_start;
                return map_ing.dest_start + source_diff;
            }
        }

        source
    }
}

fn main() {
    let input = fs
        ::read_to_string("./input.txt")
        .expect("Should have been able to read the file")
        .replace("\r", "");

    let input_lines: Vec<&str> = input.split("\n\n").collect();

    println!("Files Lines {}", input_lines.len());

    if input_lines.len() == 8 {
        let seeds: Vec<i64> = input_lines[0]
            .trim()
            .strip_prefix("seeds: ")
            .unwrap_or_default()
            .split(" ")
            .filter_map(|s| s.parse().ok())
            .collect();

        let seed_to_soil = Map::to_map(input_lines[1], "seed-to-soil map:\n");
        println!("Seed to soil: {:?}", seed_to_soil);
        let soil_to_fertilizer = Map::to_map(input_lines[2], "soil-to-fertilizer map:\n");
        let fertilizer_to_water = Map::to_map(input_lines[3], "fertilizer-to-water map:\n");
        let water_to_light = Map::to_map(input_lines[4], "water-to-light map:\n");
        let light_to_temperature = Map::to_map(input_lines[5], "light-to-temperature map:\n");
        let temperature_to_humidity = Map::to_map(input_lines[6], "temperature-to-humidity map:\n");
        let humidity_to_location = Map::to_map(input_lines[7], "humidity-to-location map:\n");

        println!(
            "Seeds:{:?}\nseed_to_soil:{:?}\nsoil_to_fertilizer:{:?}\nfertilizer_to_water:{:?}\nwater_to_light:{:?}\nlight_to_temperature:{:?}\ntemperature_to_humidity:{:?}\nhumidity_to_location:{:?}",
            seeds,
            seed_to_soil,
            soil_to_fertilizer,
            fertilizer_to_water,
            water_to_light,
            light_to_temperature,
            temperature_to_humidity,
            humidity_to_location
        );

        let mut min_loc: i64 = i64::MAX;

        for seed in seeds.clone() {
            let soil = seed_to_soil.get_map_value(seed);
            let fertilizer = soil_to_fertilizer.get_map_value(soil);
            let water = fertilizer_to_water.get_map_value(fertilizer);
            let light = water_to_light.get_map_value(water);
            let temperature = light_to_temperature.get_map_value(light);
            let humidity = temperature_to_humidity.get_map_value(temperature);
            let location = humidity_to_location.get_map_value(humidity);
            println!(
                "Seed {}, soil {}, fertilizer {}, water {}, light {}, temperature {}, humidity {}, location {}",
                seed,
                soil,
                fertilizer,
                water,
                light,
                temperature,
                humidity,
                location
            );
            min_loc = min_loc.min(location);
        }

        println!("Smallest location is: {}", min_loc);
    }
}
