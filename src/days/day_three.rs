use std::collections::HashMap;

enum WireSegment {
    Up(u16),
    Down(u16),
    Left(u16),
    Right(u16),
}

struct Wire(Vec<WireSegment>);

impl Wire {
    fn init() -> (Self, Self) {
        let raw_data = std::fs::read_to_string("./inputs/day_three").unwrap();
        let raw_data: Vec<String> = raw_data.split("\n").map(|s| s.into()).collect();

        (Wire(Wire::parse_wire(raw_data[0].clone())), Wire(Wire::parse_wire(raw_data[1].clone())))
    }

    fn parse_wire(wire_data: String) -> Vec<WireSegment> {
        wire_data
            .split(",")
            .map(|wire_seg| {
                let dist = wire_seg.get(1..).unwrap().parse::<u16>().unwrap();
                let dir = wire_seg.as_bytes().get(0..1).unwrap();
                match dir[0] as char {
                    'U' => WireSegment::Up(dist),
                    'D' => WireSegment::Down(dist),
                    'L' => WireSegment::Left(dist),
                    'R' => WireSegment::Right(dist),
                    _ => panic!("Direction not correct on: {:?}", wire_data),
                }
            })
            .collect()
    }
}

#[derive(Clone, Copy)]
enum PlotCell {
    WireOne,
    WireTwo,
}

struct WirePlot {
    plot: HashMap<(isize, isize), Vec<PlotCell>>,
    working_endpoint: (isize, isize),
}

impl WirePlot {
    fn init() -> Self {
        Self { plot: HashMap::new(), working_endpoint: (0, 0) }
    }

    fn plot(&mut self, wire: &Wire, wire_num: PlotCell) {
        self.working_endpoint = (0, 0);

        wire.0.iter().for_each(|segment| {
            let start = self.working_endpoint;

            match segment {
                WireSegment::Up(dist) => {
                    for y in start.1..(start.1 + *dist as isize) {
                        let cell = self.plot.get_mut(&(start.0, y));
                        match cell {
                            Some(cell) => cell.push(wire_num),
                            None => {
                                self.plot.insert((start.0, y), vec![wire_num]);
                            }
                        }
                    }
                    self.working_endpoint.1 += *dist as isize;
                }
                WireSegment::Down(dist) => {
                    for y in (start.1 - *dist as isize)..start.1 {
                        let cell = self.plot.get_mut(&(start.0, y));
                        match cell {
                            Some(cell) => cell.push(wire_num),
                            None => {
                                self.plot.insert((start.0, y), vec![wire_num]);
                            }
                        }
                    }
                    self.working_endpoint.1 -= *dist as isize;
                }
                WireSegment::Left(dist) => {
                    for x in (start.0 - *dist as isize)..start.0 {
                        let cell = self.plot.get_mut(&(x, start.1));
                        match cell {
                            Some(cell) => cell.push(wire_num),
                            None => {
                                self.plot.insert((x, start.1), vec![wire_num]);
                            }
                        }
                    }
                    self.working_endpoint.0 -= *dist as isize;
                }
                WireSegment::Right(dist) => {
                    for x in start.0..(start.0 + *dist as isize) {
                        let cell = self.plot.get_mut(&(x, start.1));
                        match cell {
                            Some(cell) => cell.push(wire_num),
                            None => {
                                self.plot.insert((x, start.1), vec![wire_num]);
                            }
                        }
                    }
                    self.working_endpoint.0 += *dist as isize;
                }
            }
        });
    }

    fn get_intersects(&self) -> Vec<(isize, isize)> {
        let mut intersects: Vec<(isize, isize)> = Vec::new();
        self.plot.iter().for_each(|(loc, cell)| {
            if cell.len() > 1 {
                let (mut i, mut j) = (0, 0);
                cell.iter().for_each(|wire| match wire {
                    PlotCell::WireOne => i += 1,
                    PlotCell::WireTwo => j += 1,
                });
                if i != 0 && j != 0 {
                    intersects.push(*loc)
                }
            }
        });
        intersects
    }
}

fn closest_distance(points: Vec<(isize, isize)>) -> usize {
    let mut closest_distance: usize = usize::max_value();

    for point in points {
        let dist = point.0.abs() as usize + point.1.abs() as usize;
        if dist < closest_distance {
            closest_distance = dist;
        }
    }

    closest_distance
}

pub fn part_one() {
    let wires = Wire::init();
    let mut plot = WirePlot::init();
    plot.plot(&wires.0, PlotCell::WireOne);
    plot.plot(&wires.1, PlotCell::WireTwo);
    println!("{:?}", closest_distance(plot.get_intersects()));
}

pub fn part_two() {}
