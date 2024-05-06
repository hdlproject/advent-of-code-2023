use std::fs::File;
use std::hash::Hash;
use std::io::{BufRead, BufReader};

pub struct Day {
    seeds: Vec<u64>,
    seed_soils: Vec<Vec<u64>>,
    soil_fers: Vec<Vec<u64>>,
    fer_waters: Vec<Vec<u64>>,
    water_lights: Vec<Vec<u64>>,
    light_temps: Vec<Vec<u64>>,
    temp_hums: Vec<Vec<u64>>,
    hum_locs: Vec<Vec<u64>>,
}

impl Day {
    pub fn new(input_file: &str) -> Self {
        let file = File::open(input_file).expect("should be able to open the file");
        let reader = BufReader::new(file);

        let mut seeds = Vec::new();
        let mut seed_soils = Vec::new();
        let mut soil_fers = Vec::new();
        let mut fer_waters = Vec::new();
        let mut water_lights = Vec::new();
        let mut light_temps = Vec::new();
        let mut temp_hums = Vec::new();
        let mut hum_locs = Vec::new();
        let mut step: u32 = Default::default();
        for line_res in reader.lines() {
            let line = line_res.expect("should be able to read the string");

            if line.is_empty() {
                continue;
            }

            if line.contains("seeds:") {
                step = 1;
            } else if line.contains("seed-to-soil map:") {
                step = 2;
                continue;
            } else if line.contains("soil-to-fertilizer map:") {
                step = 3;
                continue;
            } else if line.contains("fertilizer-to-water map:") {
                step = 4;
                continue;
            } else if line.contains("water-to-light map:") {
                step = 5;
                continue;
            } else if line.contains("light-to-temperature map:") {
                step = 6;
                continue;
            } else if line.contains("temperature-to-humidity map:") {
                step = 7;
                continue;
            } else if line.contains("humidity-to-location map:") {
                step = 8;
                continue;
            }

            match step {
                1 => {
                    let parts = line.split(":").filter(|x| !x.is_empty()).collect::<Vec<&str>>();
                    let numbers = parts[1].split(" ").filter(|x| !x.is_empty()).collect::<Vec<&str>>();
                    for number in numbers {
                        seeds.push(number.parse::<u64>().expect("should be number"));
                    }
                }
                2 | 3 | 4 | 5 | 6 | 7 | 8 => {
                    let numbers = line.split(" ").filter(|x| !x.is_empty()).collect::<Vec<&str>>();
                    let mut numbers_vec: Vec<u64> = Vec::new();
                    for number in numbers {
                        numbers_vec.push(number.parse::<u64>().expect("should be number"));
                    }

                    match step {
                        2 => {
                            seed_soils.push(numbers_vec);
                        }
                        3 => {
                            soil_fers.push(numbers_vec);
                        }
                        4 => {
                            fer_waters.push(numbers_vec);
                        }
                        5 => {
                            water_lights.push(numbers_vec);
                        }
                        6 => {
                            light_temps.push(numbers_vec);
                        }
                        7 => {
                            temp_hums.push(numbers_vec);
                        }
                        8 => {
                            hum_locs.push(numbers_vec);
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }


        Self {
            seeds,
            seed_soils,
            soil_fers,
            fer_waters,
            water_lights,
            light_temps,
            temp_hums,
            hum_locs,
        }
    }

    pub fn solve(&self) -> i64 {
        let mut min_loc: i64 = Default::default();

        let stages: Vec<&Vec<Vec<u64>>> = vec![
            &self.seed_soils,
            &self.soil_fers,
            &self.fer_waters,
            &self.water_lights,
            &self.light_temps,
            &self.temp_hums,
            &self.hum_locs,
        ];

        let mut locs: Vec<u64> = Default::default();
        for seed in &self.seeds {
            let mut loc: u64 = *seed;
            for stage in &stages {
                for stage_area in *stage {
                    if loc >= stage_area[1] && loc < stage_area[1] + stage_area[2] {
                        loc = stage_area[0] + (loc - stage_area[1]);
                        break;
                    }
                }
            }
            locs.push(loc)
        }

        min_loc = *locs.iter().min().expect("should not be empty") as i64;

        min_loc
    }

    pub fn solve2_bruteforce(&self) -> i64 {
        let mut min_loc: i64 = Default::default();

        let stages: Vec<&Vec<Vec<u64>>> = vec![
            &self.seed_soils,
            &self.soil_fers,
            &self.fer_waters,
            &self.water_lights,
            &self.light_temps,
            &self.temp_hums,
            &self.hum_locs,
        ];

        let mut locs: Vec<u64> = Default::default();
        let mut initial_seed: u64 = Default::default();
        for (index, seed) in self.seeds.iter().enumerate() {
            if index % 2 == 0 {
                initial_seed = *seed;
                continue;
            }

            for mut loc in initial_seed..(initial_seed + *seed) {
                for stage in &stages {
                    for stage_area in *stage {
                        if loc >= stage_area[1] && loc < stage_area[1] + stage_area[2] {
                            loc = stage_area[0] + (loc - stage_area[1]);
                            break;
                        }
                    }
                }
                locs.push(loc)
            }
        }

        min_loc = *locs.iter().min().expect("should not be empty") as i64;

        min_loc
    }

    fn get_range(&self, loc_range: &Vec<u64>, stage_areas: &Vec<Vec<u64>>) -> Vec<Vec<u64>> {
        let mut new_loc_ranges: Vec<Vec<u64>> = Default::default();

        let mut stage_area: Vec<u64> = Default::default();
        for item in stage_areas {
            if loc_range[0] >= item[1] && loc_range[0] < item[1] + item[2] {
                stage_area = item.clone();
                break;
            }
        }

        if stage_area.is_empty() {
            stage_area = vec![loc_range[0], loc_range[0], loc_range[1] - loc_range[0] + 1]
        }

        if loc_range[0] >= stage_area[1] && loc_range[1] < stage_area[1] + stage_area[2] {
            let new_loc_range: Vec<u64> = vec!(stage_area[0] + (loc_range[0] - stage_area[1]), stage_area[0] + (loc_range[1] - stage_area[1]));
            new_loc_ranges.push(new_loc_range);
        } else {
            let sub_arr_left: Vec<u64> = vec!(loc_range[0], stage_area[1] + stage_area[2] - 1);
            let sub_arr_right: Vec<u64> = vec!(stage_area[1] + stage_area[2], loc_range[1]);

            for new_loc_range in self.get_range(&sub_arr_left, stage_areas) {
                new_loc_ranges.push(new_loc_range);
            }
            for new_loc_range in self.get_range(&sub_arr_right, stage_areas) {
                new_loc_ranges.push(new_loc_range);
            }
        }

        new_loc_ranges
    }

    fn complete_range(&self, ranges: &Vec<Vec<u64>>) -> Vec<Vec<u64>> {
        let mut ordered_range = ranges.clone();
        ordered_range.sort_by(|a, b| a[1].partial_cmp(&b[1]).unwrap());

        let mut new_ranges: Vec<Vec<u64>> = Default::default();
        if ordered_range[0][1] != 0 {
            new_ranges.push(vec![0, 0, ordered_range[0][1]]);
        }
        let mut prev_range = ordered_range[0].clone();
        for range in &ordered_range[1..ordered_range.len()] {
            new_ranges.push(prev_range.clone());

            if prev_range[1] + prev_range[2] != range[1] {
                new_ranges.push(vec![prev_range[1] + prev_range[2], prev_range[1] + prev_range[2], range[1] - (prev_range[1] + prev_range[2])]);
            }

            prev_range = range.clone();
        }
        new_ranges.push(ordered_range[ordered_range.len() - 1].clone());

        return new_ranges;
    }

    pub fn solve2(&self) -> i64 {
        let mut min_loc: i64 = Default::default();

        let complete_seed_soils = self.complete_range(&self.seed_soils);
        let complete_soil_fers = self.complete_range(&self.soil_fers);
        let complete_fer_waters = self.complete_range(&self.fer_waters);
        let complete_water_lights = self.complete_range(&self.water_lights);
        let complete_light_temps = self.complete_range(&self.light_temps);
        let complete_temp_hums = self.complete_range(&self.temp_hums);
        let complete_hum_locs = self.complete_range(&self.hum_locs);

        let stages: Vec<&Vec<Vec<u64>>> = vec![
            &complete_seed_soils,
            &complete_soil_fers,
            &complete_fer_waters,
            &complete_water_lights,
            &complete_light_temps,
            &complete_temp_hums,
            &complete_hum_locs,
        ];

        let mut initial_seed: u64 = Default::default();
        let mut loc_ranges: Vec<Vec<u64>> = Default::default();
        for (index, seed) in self.seeds.iter().enumerate() {
            if index % 2 == 0 {
                initial_seed = *seed;
                continue;
            } else {
                loc_ranges.push(vec![initial_seed, initial_seed + *seed])
            }
        }

        for stage in stages {
            let mut new_loc_ranges: Vec<Vec<u64>> = Default::default();
            for loc_range in loc_ranges.iter() {
                for item in self.get_range(loc_range, stage) {
                    new_loc_ranges.push(item);
                }
            }
            loc_ranges = new_loc_ranges;
        }

        let mut ordered_range = loc_ranges.clone();
        ordered_range.sort_by(|a, b| a[1].partial_cmp(&b[1]).unwrap());

        min_loc = ordered_range[0][0] as i64;
        min_loc
    }
}