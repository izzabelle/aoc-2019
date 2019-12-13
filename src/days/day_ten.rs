use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
enum Location {
    Asteroid,
    Empty,
}

impl std::convert::From<char> for Location {
    fn from(chr: char) -> Location {
        match chr {
            '.' => Location::Empty,
            '#' => Location::Asteroid,
            _ => panic!("invalid input!"),
        }
    }
}

#[derive(Debug)]
struct AsteroidMap {
    data: Vec<Location>,
    width: usize,
    height: usize,
}

impl std::ops::Index<(usize, usize)> for AsteroidMap {
    type Output = Location;

    fn index(&self, idx: (usize, usize)) -> &Location {
        &self.data[self.width * idx.1 + idx.0]
    }
}

impl std::ops::IndexMut<(usize, usize)> for AsteroidMap {
    fn index_mut(&mut self, idx: (usize, usize)) -> &mut Location {
        &mut self.data[self.width * idx.1 + idx.0]
    }
}

impl AsteroidMap {
    fn init(width: usize, height: usize, input: &'static str) -> Self {
        let input = std::fs::read_to_string(input).unwrap();
        let mut data: Vec<Location> = Vec::new();
        input.lines().for_each(|line| line.chars().for_each(|chr| data.push(Location::from(chr))));
        AsteroidMap { height, width, data }
    }

    fn asteroids_as_vec(&self) -> Vec<(usize, usize)> {
        let mut vec: Vec<(usize, usize)> = Vec::new();
        for x in 0..self.width {
            for y in 0..self.height {
                if self[(x, y)] == Location::Asteroid {
                    vec.push((x, y));
                }
            }
        }
        vec
    }

    fn find_angles(&self) -> HashMap<(usize, usize), Vec<f64>> {
        let mut map: HashMap<(usize, usize), Vec<f64>> = HashMap::new();

        for station in self.asteroids_as_vec() {
            let mut angles: Vec<f64> = Vec::new();

            for asteroid in self.asteroids_as_vec() {
                if asteroid == station {
                    continue;
                }

                let (x_0, y_0, x_1, y_1) =
                    (station.0 as f64, station.1 as f64, asteroid.0 as f64, asteroid.1 as f64);

                angles.push((y_1 - y_0) / (x_1 - x_0));
            }

            map.insert(station, angles);
        }
        map
    }
}

fn angle_counts(map: HashMap<(usize, usize), Vec<f64>>) -> HashMap<(usize, usize), usize> {
    let mut result_map: HashMap<(usize, usize), usize> = HashMap::new();

    for (station, angles) in map {
        let mut valid_angles: Vec<f64> = Vec::new();

        for angle in angles {
            let mut is_unique = true;

            for valid_angle in valid_angles.clone() {
                if angle == valid_angle {
                    is_unique = false;
                }
            }

            if is_unique {
                valid_angles.push(angle);
            }
        }

        result_map.insert(station, valid_angles.len());
    }

    result_map
}

pub fn part_one() {
    let timer = std::time::Instant::now();

    let asteroid_map = AsteroidMap::init(33, 33, "./inputs/day_ten");
    let results = angle_counts(asteroid_map.find_angles());
    let mut highest = 0;

    for (_, count) in results {
        if count > highest {
            highest = count;
        }
    }

    println!("highest count: {}", highest);

    println!("execution time: {}ms", timer.elapsed().as_millis());
}

pub fn part_two() {
    let timer = std::time::Instant::now();

    //code here

    println!("execution time: {}ms", timer.elapsed().as_millis());
}
