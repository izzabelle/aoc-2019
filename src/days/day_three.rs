use std::collections::HashMap;

// represent the segments by their changes in location on the board
#[derive(Clone, PartialEq, Eq)]
struct Segment {
    grid_delta: (isize, isize),
}

impl std::convert::From<&String> for Segment {
    fn from(segment: &String) -> Segment {
        let dist = segment.get(1..).unwrap().parse::<isize>().unwrap();

        match segment.get(0..1).unwrap() {
            "U" => Self { grid_delta: (0, dist) },
            "D" => Self { grid_delta: (0, 0 - dist) },
            "R" => Self { grid_delta: (dist, 0) },
            "L" => Self { grid_delta: (0 - dist, 0) },
            _ => panic!("invald data: {}", segment),
        }
    }
}

// represent both wires in a structure of vecs of segments
#[derive(Clone)]
struct Wires {
    one: Vec<Segment>,
    two: Vec<Segment>,
}

impl Wires {
    // parse grid deltas from string
    fn parse(input: String) -> Self {
        let wires: Vec<String> = input.clone().split("\n").map(|s| s.into()).collect();
        let (wire_one, wire_two) = (wires[0].to_string(), wires[1].to_string());

        let one: Vec<Segment> = wire_one
            .split(",")
            .map(|s| s.to_string())
            .map(|segment| Segment::from(&segment))
            .collect();
        let two: Vec<Segment> = wire_two
            .split(",")
            .map(|s| s.to_string())
            .map(|segment| Segment::from(&segment))
            .collect();

        Self { one, two }
    }
}

// which wire
enum WireNum {
    One,
    Two,
}

// a single cell in the map
struct Grid {
    wire: WireNum,
    length_to_cell: usize,
}

// to contain info for plotting
struct Plotter {
    current_segment: Segment,
    total_written: usize,
    complete: bool,
}

// represents the plot, wires, and the supplemental tools to plot the wires too it
struct Plot {
    map: HashMap<(isize, isize), Grid>,
    wires: Wires,
    one: Plotter,
    two: Plotter,
}

impl Plotter {
    // check segment if complete, if so pop new from wire return true
    fn check_segment(&mut self, plot: &mut Plot, wire: WireNum) -> bool {
        if self.current_segment == Segment{(0, 0)} {
            match wire {
                WireNum::One => match plot.wires.one.pop() {
                    Some(segment) => {
                        self.current_segment = segment;
                        false
                    }
                    None => true,
                },
                WireNum::Two => {}
            }
        } else {
            false
        }
    }
}

impl Plot {
    // initialize the struct
    fn init(wires: Wires) -> Self {
        let mut wires = wires.clone();
        let map: HashMap<(isize, isize), Grid> = HashMap::new();

        wires.one.reverse();
        wires.two.reverse();

        let one = Plotter {
            current_segment: wires.one.pop().unwrap(),
            total_written: 0,
            complete: false,
        };
        let two = Plotter {
            current_segment: wires.two.pop().unwrap(),
            total_written: 0,
            complete: false,
        };

        Self { map, wires, one, two }
    }

    // plots the wires to the map
    fn run(&mut self) {
        // init

        loop {
            // plot wire one
            if !self.one.complete {
                // check segment
            }

            // plot wire two
            if !self.two.complete {}

            // break if complete
            if self.one.complete && self.two.complete {
                break;
            }
        }
    }
}

pub fn part_one() {
    let wires = Wires::parse(std::fs::read_to_string("./inputs/day_three").unwrap());
    let plot = Plot::init(wires);
}

pub fn part_two() {}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &'static str =
        "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83";
    const TEST_SOLUTION_PART_ONE: usize = 159;
    const TEST_SOLUTION_PART_TWO: usize = 610;

    #[test]
    fn part_one() {
        let &mut wires = Wires::parse(TEST_INPUT);
        let plot = Plot::init(wires);
        unimplemented!();
    }

    #[test]
    fn part_two() {
        unimplemented!()
    }
}
