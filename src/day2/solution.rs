use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Day {
    games: Vec<Vec<HashMap<String, u64>>>,
    game_threshold: HashMap<String, u64>,
}

impl Day {
    pub fn new(input_file: &str) -> Self {
        let file = File::open(input_file).expect("should be able to open the file");
        let reader = BufReader::new(file);

        let mut games_vec = Vec::new();
        for line_res in reader.lines() {
            let line = line_res.expect("should be able to read the string");
            let order_and_games = line.split(":").filter(|&x| !x.is_empty()).collect::<Vec<&str>>();
            let games = order_and_games[1].split(";").filter(|&x| !x.is_empty()).collect::<Vec<&str>>();

            let mut game_vec = Vec::new();
            for game in games {
                let cubes = game.split(",").filter(|&x| !x.is_empty()).collect::<Vec<&str>>();

                let mut cube_map = HashMap::new();
                for cube in cubes {
                    let items = cube.split(" ").filter(|&x| !x.is_empty() && x != " ").collect::<Vec<&str>>();

                    cube_map.insert(items[1].to_owned(), items[0].parse::<u64>().expect("expect number string"));
                }
                game_vec.push(cube_map);
            }
            games_vec.push(game_vec);
        }


        Self {
            games: games_vec,
            game_threshold: HashMap::from([
                ("red".into(), 12),
                ("green".into(), 13),
                ("blue".into(), 14),
            ]),
        }
    }

    pub fn solve(&self) -> u64 {
        let mut total: u64 = Default::default();

        for (index, game) in self.games.iter().enumerate() {
            let mut invalid_game: bool = false;
            for turn in game {
                for (color, threshold) in &self.game_threshold {
                    if turn.contains_key(color) && turn.get(color).expect("expect get non empty map value") > &threshold {
                        invalid_game = true;
                        break;
                    }
                }
                if invalid_game {
                    break;
                }
            }
            if !invalid_game {
                let index_u64: u64 = index as u64;
                total += index_u64 + 1;
            }
        }

        total
    }

    pub fn solve2(&self) -> u64 {
        let mut total: u64 = Default::default();

        for game in &self.games {
            let mut color_max: HashMap<String, u64> = HashMap::from([
                ("red".into(), 0),
                ("green".into(), 0),
                ("blue".into(), 0),
            ]);
            for turn in game {
                for (color, number) in turn {
                    if color_max.get(color).expect("expect get non empty map value") < number {
                        color_max.insert(color.to_string(), *number);
                    }
                }
            }
            let mut sub_total: u64 = 1;
            for (color, number) in color_max {
                sub_total *= number;
            }
            total += sub_total;
        }

        total
    }
}
