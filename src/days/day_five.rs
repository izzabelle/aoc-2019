use advent_of_code::IntCodeComputer;

pub fn part_one() {
    let timer = std::time::Instant::now();

    let mut comp = IntCodeComputer::init(std::path::PathBuf::from("./inputs/day_five"), vec![1]);
    loop {
        comp.process();
        if comp.halted {
            break;
        }
    }

    println!("diagnostic code: {}", comp.output_buffer[comp.output_buffer.len() - 1]);

    println!("execution time: {}us", timer.elapsed().as_micros());
}

pub fn part_two() {
    let timer = std::time::Instant::now();

    let mut comp = IntCodeComputer::init(std::path::PathBuf::from("./inputs/day_five"), vec![5]);
    loop {
        comp.process();
        if comp.halted {
            break;
        }
    }

    println!("diagnostic code: {}", comp.output_buffer[comp.output_buffer.len() - 1]);

    println!("execution time: {}us", timer.elapsed().as_micros());
}
