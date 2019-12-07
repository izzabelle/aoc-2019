#[cfg(test)]
use crate::IntCodeComputer;

#[test]
fn day_two_part_one() {
    let mut comp = IntCodeComputer::init(std::path::PathBuf::from("./inputs/day_two"), Vec::new());

    comp.memory[1] = 12;
    comp.memory[2] = 02;

    loop {
        comp.process();
        if comp.halted {
            break;
        }
    }

    assert_eq!(10566835, comp.memory[0]);
}

#[test]
fn day_two_part_two() {
    let mut comp = IntCodeComputer::init(std::path::PathBuf::from("./inputs/day_two"), Vec::new());

    let mut result: isize = 0;

    for noun in 0..99 {
        for verb in 0..99 {
            comp.memory[1] = noun;
            comp.memory[2] = verb;

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

    assert_eq!(result, 2347);
}

#[test]
fn day_five_part_one() {
    let mut comp = IntCodeComputer::init(std::path::PathBuf::from("./inputs/day_five"), vec![1]);
    loop {
        comp.process();
        if comp.halted {
            break;
        }
    }

    assert_eq!(comp.output_buffer[comp.output_buffer.len() - 1], 12440243);
}

#[test]
fn day_five_part_two() {
    let mut comp = IntCodeComputer::init(std::path::PathBuf::from("./inputs/day_five"), vec![5]);
    loop {
        comp.process();
        if comp.halted {
            break;
        }
    }

    assert_eq!(comp.output_buffer[comp.output_buffer.len() - 1], 15486302);
}
