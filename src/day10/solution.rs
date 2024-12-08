use std::cmp;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Day {
    map: Vec<Vec<char>>,
    start_position: (usize, usize),
}

impl Day {
    pub fn new(input_file: &str) -> Self {
        let file = File::open(input_file).unwrap();
        let reader = BufReader::new(file);
        let lines = reader.lines();

        let mut map: Vec<Vec<char>> = Vec::new();
        let mut start_position = (0, 0);
        let mut y: usize = Default::default();
        for line_res in lines {
            let line = line_res.unwrap();
            let parts: Vec<char> = line.chars().collect();
            map.push(parts.clone());

            if let Some(x) = parts.iter().position(|&x| x == 'S') {
                start_position = (y, x);
            }
            y += 1;
        }

        Day {
            map,
            start_position,
        }
    }

    pub fn solve(&self) -> u64 {
        let mut neighbors: Vec<((usize, usize), char)> = Vec::new();
        if self.start_position.0 > 0
            && !vec!['L', 'J', '-', '.']
                .contains(&self.map[self.start_position.0 - 1][self.start_position.1])
        {
            neighbors.push(((self.start_position.0 - 1, self.start_position.1), 'u'));
        }
        if self.start_position.0 < self.map.len() - 1
            && !vec!['F', '7', '-', '.']
                .contains(&self.map[self.start_position.0 + 1][self.start_position.1])
        {
            neighbors.push(((self.start_position.0 + 1, self.start_position.1), 'd'));
        }
        if self.start_position.1 > 0
            && !vec!['7', 'J', '|', '.']
                .contains(&self.map[self.start_position.0][self.start_position.1 - 1])
        {
            neighbors.push(((self.start_position.0, self.start_position.1 - 1), 'l'));
        }
        if self.start_position.1 < self.map[0].len() - 1
            && !vec!['F', 'L', '|', '.']
                .contains(&self.map[self.start_position.0][self.start_position.1 + 1])
        {
            neighbors.push(((self.start_position.0, self.start_position.1 + 1), 'r'));
        }

        let mut max_count: u64 = Default::default();
        let mut already_traversed_neighbor_dir: Vec<char> = Vec::new();
        for neighbor in neighbors {
            if already_traversed_neighbor_dir.contains(&neighbor.1) {
                continue;
            }

            let mut count: u64 = Default::default();
            let mut next_pos_and_dir: ((usize, usize), char) = neighbor;
            let mut stop = false;
            while !stop {
                count += 1;

                next_pos_and_dir = self.next_position(
                    next_pos_and_dir,
                    self.map[next_pos_and_dir.0 .0][next_pos_and_dir.0 .1],
                );
                let (next_pos, next_dir) = next_pos_and_dir;

                stop = (next_dir == 's') || (self.map[next_pos.0][next_pos.1] == 'S');

                if self.map[next_pos.0][next_pos.1] == 'S' {
                    match next_dir {
                        'r' => already_traversed_neighbor_dir.push('l'),
                        'l' => already_traversed_neighbor_dir.push('r'),
                        'u' => already_traversed_neighbor_dir.push('d'),
                        'd' => already_traversed_neighbor_dir.push('u'),
                        _ => {
                            panic!("Unexpected direction");
                        }
                    }
                }

                already_traversed_neighbor_dir.push(neighbor.1);
            }

            max_count = cmp::max(max_count, count);
        }

        (max_count as f64 / 2.0).ceil() as u64
    }

    fn next_position(&self, node: ((usize, usize), char), pipe: char) -> ((usize, usize), char) {
        let ((y, x), dir) = node;

        let mut pos_and_dir: ((i32, i32), char) = Default::default();
        match pipe {
            'F' => match dir {
                'l' => pos_and_dir = (((y + 1) as i32, x as i32), 'd'),
                'u' => pos_and_dir = ((y as i32, (x + 1) as i32), 'r'),
                _ => {
                    panic!("Unexpected direction");
                }
            },
            '7' => match dir {
                'r' => pos_and_dir = (((y + 1) as i32, x as i32), 'd'),
                'u' => pos_and_dir = ((y as i32, (x - 1) as i32), 'l'),
                _ => {
                    panic!("Unexpected direction");
                }
            },
            'J' => match dir {
                'r' => pos_and_dir = (((y - 1) as i32, x as i32), 'u'),
                'd' => pos_and_dir = ((y as i32, (x - 1) as i32), 'l'),
                _ => {
                    panic!("Unexpected direction");
                }
            },
            'L' => match dir {
                'l' => pos_and_dir = (((y - 1) as i32, x as i32), 'u'),
                'd' => pos_and_dir = ((y as i32, (x + 1) as i32), 'r'),
                _ => {
                    panic!("Unexpected direction");
                }
            },
            '|' => match dir {
                'u' => pos_and_dir = (((y - 1) as i32, x as i32), 'u'),
                'd' => pos_and_dir = (((y + 1) as i32, x as i32), 'd'),
                _ => {
                    panic!("Unexpected direction");
                }
            },
            '-' => match dir {
                'r' => pos_and_dir = ((y as i32, (x + 1) as i32), 'r'),
                'l' => pos_and_dir = ((y as i32, (x - 1) as i32), 'l'),
                _ => {
                    panic!("Unexpected direction");
                }
            },
            _ => {
                panic!("Unexpected direction");
            }
        }

        if pos_and_dir.0 .0 < 0 || pos_and_dir.0 .0 >= self.map.len() as i32 {
            return ((0, 0), 's');
        }

        if pos_and_dir.0 .1 < 0 || pos_and_dir.0 .1 >= self.map[0].len() as i32 {
            return ((0, 0), 's');
        }

        (
            (pos_and_dir.0 .0 as usize, pos_and_dir.0 .1 as usize),
            pos_and_dir.1,
        )
    }
}
