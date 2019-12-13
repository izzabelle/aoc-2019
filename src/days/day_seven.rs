use advent_of_code::IntCodeComputer;

fn run(comp: &mut IntCodeComputer, input_one: isize, input_two: isize) -> isize {
    comp.input_buffer.push(input_one);
    comp.input_buffer.push(input_two);

    loop {
        comp.process();
        if comp.halted {
            break;
        }
    }

    comp.output_buffer[0]
}

fn try_amplifier_signals(comp: &mut IntCodeComputer) -> isize {
    let mut max_signal = 0;

    for a in 0..5 {
        for b in 0..5 {
            if a == b {
                continue;
            }
            for c in 0..5 {
                if a == c || b == c {
                    continue;
                }
                for d in 0..5 {
                    if a == d || b == d || c == d {
                        continue;
                    }
                    for e in 0..5 {
                        if a == e || b == e || c == e || d == e {
                            continue;
                        }
                        comp.reset();
                        let mut signal = run(comp, 0, a);
                        comp.reset();
                        signal = run(comp, signal, b);
                        comp.reset();
                        signal = run(comp, signal, c);
                        comp.reset();
                        signal = run(comp, signal, d);
                        comp.reset();
                        signal = run(comp, signal, e);

                        println!("[{}, {}, {}, {}, {}] = {}", a, b, c, d, e, signal);
                        if signal > max_signal {
                            max_signal = comp.output_buffer[0];
                        }
                    }
                }
            }
        }
    }

    max_signal
}

pub fn part_one() {
    let timer = std::time::Instant::now();

    let mut comp =
        IntCodeComputer::init(std::path::PathBuf::from("./inputs/day_seven"), Vec::new());

    println!("max signal: {}", try_amplifier_signals(&mut comp));

    println!("execution time: {}ms", timer.elapsed().as_millis());
}

fn run2(comp: &mut IntCodeComputer, input_one: isize, input_two: isize) -> isize {
    comp.input_buffer.push(input_one);
    comp.input_buffer.push(input_two);

    loop {
        comp.process();
        if comp.halted || comp.output_buffer.len() > 0 {
            break;
        }
    }

    comp.output_buffer.pop().unwrap()
}

fn try_amplifier_signals2(comps: [&mut IntCodeComputer; 5]) -> isize {
    let mut max_signal = 0;

    for a in 5..10 {
        for b in 5..10 {
            if a == b {
                continue;
            }
            for c in 5..10 {
                if a == c || b == c {
                    continue;
                }
                for d in 5..10 {
                    if a == d || b == d || c == d {
                        continue;
                    }
                    for e in 5..10 {
                        if a == e || b == e || c == e || d == e {
                            continue;
                        }

                        let mut signal = run2(comps[0], a, 0);

                        loop {
                            signal = run2(comps[1], b, signal);
                            signal = run2(comps[2], c, signal);
                            signal = run2(comps[3], d, signal);
                            signal = run2(comps[4], e, signal);
                            if comps[4].halted {
                                break;
                            }
                            signal = run2(comps[0], a, signal);
                        }

                        println!("[{}, {}, {}, {}, {}] = {}", a, b, c, d, e, signal);
                        if signal > max_signal {
                            max_signal = signal;
                        }
                    }
                }
            }
        }
    }

    max_signal
}

pub fn part_two() {
    let timer = std::time::Instant::now();

    let mut comp =
        IntCodeComputer::init(std::path::PathBuf::from("./inputs/day_seven"), Vec::new());
    let mut comp_one =
        IntCodeComputer::init(std::path::PathBuf::from("./inputs/day_seven"), Vec::new());
    let mut comp_two =
        IntCodeComputer::init(std::path::PathBuf::from("./inputs/day_seven"), Vec::new());
    let mut comp_three =
        IntCodeComputer::init(std::path::PathBuf::from("./inputs/day_seven"), Vec::new());
    let mut comp_four =
        IntCodeComputer::init(std::path::PathBuf::from("./inputs/day_seven"), Vec::new());

    let amps = [&mut comp, &mut comp_one, &mut comp_two, &mut comp_three, &mut comp_four];

    try_amplifier_signals2(amps);

    println!("execution time: {}ms", timer.elapsed().as_millis());
}
