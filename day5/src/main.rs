use std::{ fs, time::Instant };

#[derive(Debug)]
pub struct Mapping {
    dest_start: u128,
    source_start: u128,
    range_length: u128,
}

fn to_map(input: &str, prefix: &str) -> Vec<Mapping> {
    let mappings: Vec<Mapping> = input
        .trim()
        .strip_prefix(prefix)
        .unwrap_or_default()
        .split("\n")
        .filter_map(|f| {
            let maps: Vec<u128> = f
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

    mappings
}

fn get_map_value(mappings: &Vec<Mapping>, source: u128) -> u128 {
    for map_ing in mappings {
        if map_ing.source_start <= source && map_ing.source_start + map_ing.range_length > source {
            let source_diff = source - map_ing.source_start;
            return map_ing.dest_start + source_diff;
        }
    }

    source
}

fn main() {
    let input = fs
        ::read_to_string("./input.txt")
        .expect("Should have been able to read the file")
        .replace("\r", "");

    let input_lines: Vec<&str> = input.split("\n\n").collect();

    println!("Files Lines {}", input_lines.len());

    if input_lines.len() == 8 {
        let seeds: Vec<u128> = input_lines[0]
            .trim()
            .strip_prefix("seeds: ")
            .unwrap_or_default()
            .split(" ")
            .filter_map(|s| s.parse().ok())
            .collect();

        let seed_to_soil = to_map(input_lines[1], "seed-to-soil map:\n");
        println!("Seed to soil: {:?}", seed_to_soil);
        let soil_to_fertilizer = to_map(input_lines[2], "soil-to-fertilizer map:\n");
        let fertilizer_to_water = to_map(input_lines[3], "fertilizer-to-water map:\n");
        let water_to_light = to_map(input_lines[4], "water-to-light map:\n");
        let light_to_temperature = to_map(input_lines[5], "light-to-temperature map:\n");
        let temperature_to_humidity = to_map(input_lines[6], "temperature-to-humidity map:\n");
        let humidity_to_location = to_map(input_lines[7], "humidity-to-location map:\n");

        println!(
            "Seeds:{:?}\nseed_to_soil:{:?}\nsoil_to_fertilizer:{:?}\nfertilizer_to_water:{:?}\nwater_to_light:{:?}\nlight_to_temperature:{:?}\ntemperature_to_humidity:{:?}\nhumidity_to_location:{:?}",
            seeds,
            seed_to_soil.len(),
            soil_to_fertilizer.len(),
            fertilizer_to_water.len(),
            water_to_light.len(),
            light_to_temperature.len(),
            temperature_to_humidity.len(),
            humidity_to_location.len()
        );

        let mut min_loc: u128 = u128::MAX;

        for seed in seeds.clone() {
            let soil = get_map_value(&seed_to_soil, seed);
            let fertilizer = get_map_value(&soil_to_fertilizer, soil);
            let water = get_map_value(&fertilizer_to_water, fertilizer);
            let light = get_map_value(&water_to_light, water);
            let temperature = get_map_value(&light_to_temperature, light);
            let humidity = get_map_value(&temperature_to_humidity, temperature);
            let location = get_map_value(&humidity_to_location, humidity);
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

        let mut min_loc: u128 = u128::MAX;

        let start = Instant::now();

        if seeds.len() % 2 == 0 {
            let seed_chunks: Vec<&[u128]> = seeds.chunks_exact(2).collect();
            for seed_range in seed_chunks {
                let range = seed_range[1];
                println!("Starting with:{}, {} long, lowest: {}", seed_range[0], range, min_loc);
                for seed in seed_range[0]..seed_range[0] + seed_range[1] {
                    // println!("ID{} seed{} range{} i{}",start_seed,seed,range,i);
                    let location = get_map_value(
                        &humidity_to_location,
                        get_map_value(
                            &temperature_to_humidity,
                            get_map_value(
                                &light_to_temperature,
                                get_map_value(
                                    &water_to_light,
                                    get_map_value(
                                        &fertilizer_to_water,
                                        get_map_value(
                                            &soil_to_fertilizer,
                                            get_map_value(&seed_to_soil, seed)
                                        )
                                    )
                                )
                            )
                        )
                    );
                    min_loc = min_loc.min(location);
                }
            }
            println!("Smallest location is: {}, took {:?}", min_loc, start.elapsed());
        }
    }
}
