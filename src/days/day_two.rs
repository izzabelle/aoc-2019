use advent_of_code::IntCodeComputer;

pub fn part_one() {
    let timer = std::time::Instant::now();

    let mut comp = IntCodeComputer::init(std::path::PathBuf::from("./inputs/day_two"), Vec::new());

    comp.set_addr(1, 12);
    comp.set_addr(2, 02);

    loop {
        comp.process();
        if comp.halted {
            break;
        }
    }

    println!("memory addr 0 conts: {}", comp.memory[0]);

    println!("execution time: {}ms", timer.elapsed().as_millis());
}

pub fn part_two() {
    let timer = std::time::Instant::now();

    let mut comp = IntCodeComputer::init(std::path::PathBuf::from("./inputs/day_two"), Vec::new());

    let mut result: isize = 0;

    for noun in 0..99 {
        for verb in 0..99 {
            comp.set_addr(1, noun);
            comp.set_addr(2, verb);

            loop {
                comp.process();
                if comp.halted {
                    break;
                }
            }

            if comp.memory[0] == 19690720 {
                result = 100 * noun + verb;
                comp.reset();
                break;
            } else {
                comp.reset();
            }
        }

        if comp.memory[0] == 19690720 {
            break;
        }
    }

    println!("result: {}", result);

    println!("execution time: {}ms", timer.elapsed().as_millis());
}
