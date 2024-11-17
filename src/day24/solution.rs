use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Point {
    x: f64,
    y: f64,
    z: f64,
}

pub struct Vector {
    pos: Point,
    vel: Point,
}

pub struct Day {
    rows: Vec<Vector>,
    min_coord: f64,
    max_coord: f64,
}

impl Day {
    pub fn new(input_file: &str) -> Self {
        let file = File::open(input_file).expect("should be able to open the file");
        let reader = BufReader::new(file);


        let mut vectors: Vec<Vector> = Default::default();
        for line_res in reader.lines() {
            let line = line_res.expect("should be able to read the string");

            let line_part = line.split("@").filter(|x| !x.is_empty()).collect::<Vec<&str>>();
            let line_pos_part = line_part[0].split(",").filter(|x| !x.is_empty()).collect::<Vec<&str>>();
            let line_vel_part = line_part[1].split(",").filter(|x| !x.is_empty()).collect::<Vec<&str>>();

            let rows: Vec<Vector> = Default::default();
            let vector = Vector {
                pos: Point {
                    x: line_pos_part[0].trim().parse::<f64>().expect("should be number"),
                    y: line_pos_part[1].trim().parse::<f64>().expect("should be number"),
                    z: line_pos_part[2].trim().parse::<f64>().expect("should be number"),
                },
                vel: Point {
                    x: line_vel_part[0].trim().parse::<f64>().expect("should be number"),
                    y: line_vel_part[1].trim().parse::<f64>().expect("should be number"),
                    z: line_vel_part[2].trim().parse::<f64>().expect("should be number"),
                },
            };
            vectors.push(vector)
        }

        Self {
            rows: vectors,
            // min_coord: 7f64,
            // max_coord: 27f64,
            min_coord: 200000000000000f64,
            max_coord: 400000000000000f64,
        }
    }

    fn get_determinant(&self, a: f64, b: f64, c: f64, d: f64) -> f64 {
        return (a * d) - (b * c);
    }

    fn get_intersect_point(&self, vec1: &Vector, vec2: &Vector) -> Point {
        let div = self.get_determinant(-1f64 * vec1.vel.x, -1f64 * vec2.vel.x, -1f64 * vec1.vel.y, -1f64 * vec2.vel.y);
        if div == 0f64 {
            return Point {
                x: 0f64,
                y: 0f64,
                z: 0f64,
            };
        }

        let c1 = self.get_determinant(vec1.pos.x, vec1.pos.y, vec1.pos.x + vec1.vel.x, vec1.pos.y + vec1.vel.y);
        let c2 = self.get_determinant(vec2.pos.x, vec2.pos.y, vec2.pos.x + vec2.vel.x, vec2.pos.y + vec2.vel.y);

        let x_div = self.get_determinant(c1, c2, -1f64 * vec1.vel.x, -1f64 * vec2.vel.x);
        let y_div = self.get_determinant(c1, c2, -1f64 * vec1.vel.y, -1f64 * vec2.vel.y);

        println!("intersect {} {} {} {}", c1, c2, x_div, y_div);

        Point {
            x: x_div / div,
            y: y_div / div,
            z: 0f64,
        }
    }

    fn get_intersect_point2(&self, vec1: &Vector, vec2: &Vector) -> Point {
        let div = self.get_determinant(vec1.vel.x, vec2.vel.x, vec1.vel.y, vec2.vel.y);
        if div == 0f64 {
            return Point {
                x: 0f64,
                y: 0f64,
                z: 0f64,
            };
        }

        let c1 = self.get_determinant(vec1.pos.x, vec1.pos.y, vec1.pos.x + vec1.vel.x, vec1.pos.y + vec1.vel.y);
        let c2 = self.get_determinant(vec2.pos.x, vec2.pos.y, vec2.pos.x + vec2.vel.x, vec2.pos.y + vec2.vel.y);

        let x_div = self.get_determinant(c1, c2, vec1.vel.y, vec2.vel.y);
        let y_div = self.get_determinant(vec1.vel.x, vec2.vel.x, c1, c2);

        println!("intersect {} {} {} {}", c1, c2, x_div, y_div);

        Point {
            x: x_div / div,
            y: y_div / div,
            z: 0f64,
        }
    }

    fn get_intersect_point3(&self, vec1: &Vector, vec2: &Vector) -> Point {
        let div = self.get_determinant(vec1.vel.x, vec2.vel.x, vec1.vel.y, vec2.vel.y);
        if div == 0f64 {
            return Point {
                x: 0f64,
                y: 0f64,
                z: 0f64,
            };
        }

        let c1 = vec1.pos.y + ((vec1.pos.x.abs() / vec1.vel.x.abs()) * vec1.vel.y);
        let c2 = vec2.pos.y + ((vec2.pos.x.abs() / vec2.vel.x.abs()) * vec2.vel.y);

        let x_div = self.get_determinant(c1, c2, vec1.vel.y, vec2.vel.y);
        let y_div = self.get_determinant(vec1.vel.x, vec2.vel.x, c1, c2);

        println!("intersect {} {} {} {}", c1, c2, x_div, y_div);

        Point {
            x: x_div / div,
            y: y_div / div,
            z: 0f64,
        }
    }

    fn is_sign(&self, number: f64) -> i32 {
        if number < 0f64 {
            return -1;
        } else if number > 0f64 {
            return 1;
        }
        return 0;
    }

    fn is_future_crossed(&self, intersect: &Point, point: &Point, point_vel: &Point) -> bool {
        self.is_sign(intersect.x - point.x) == self.is_sign(point_vel.x) &&
            self.is_sign(intersect.y - point.y) == self.is_sign(point_vel.y)
    }

    pub fn solve(&self) -> u64 {
        let mut total: u64 = Default::default();

        for (index, row) in self.rows.iter().enumerate() {
            for t_row in self.rows[index..].iter() {
                let intersect: Point = self.get_intersect_point2(row, t_row);
                if intersect.x == 0f64 && intersect.y == 0f64 {
                    continue;
                }

                if !self.is_future_crossed(&intersect, &row.pos, &row.vel) ||
                    !self.is_future_crossed(&intersect, &t_row.pos, &t_row.vel) {
                    continue;
                }

                if !(self.min_coord <= intersect.x) || !(self.max_coord >= intersect.x) ||
                    !(self.min_coord <= intersect.y) || !(self.max_coord >= intersect.y) {
                    continue;
                }

                total += 1;

                // if self.is_future_crossed(&intersect, &row.pos, &row.vel) &&
                //     self.is_future_crossed(&intersect, &t_row.pos, &t_row.vel) {
                //     if (self.min_coord <= intersect.x) && (self.max_coord >= intersect.x) &&
                //         (self.min_coord <= intersect.y) && (self.max_coord >= intersect.y) {
                //         total += 1;
                //     }
                // }
            }
        }

        total
        // 13135 is too low
        // 26270 is too high
        // how to actually get C value?
    }
}
