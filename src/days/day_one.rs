pub fn part_one() {
    let mut input: Vec<u32> = Vec::new();
    std::fs::read_to_string("./inputs/day_one").unwrap().lines().for_each(|line| {
        input.push(line.parse::<u32>().unwrap());
    });
    let total_fuel: u32 = input.iter().map(|mass| mass / 3 - 2).sum();
    println!("{}", total_fuel);
}

pub fn part_two() {
    let mut input: Vec<i64> = Vec::new();
    std::fs::read_to_string("./inputs/day_one").unwrap().lines().for_each(|line| {
        input.push(line.parse::<i64>().unwrap());
    });
    let total_fuel: i64 = input.iter().map(|mass| fuel_loop(*mass)).sum();
    println!("{}", total_fuel);
}

fn fuel_loop(mass: i64) -> i64 {
    let fuel = mass / 3 - 2;
    if fuel > 0 {
        fuel + fuel_loop(fuel)
    } else {
        0
    }
}
