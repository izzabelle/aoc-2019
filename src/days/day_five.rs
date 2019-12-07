#[derive(Debug)]
struct IntCodeComputer {
    memory: Vec<isize>,
    memory_backup: Vec<isize>,
    ip: usize,
    halted: bool,
    output_buffer: Vec<isize>,
    input_buffer: Vec<isize>,
}

#[allow(dead_code)]
impl IntCodeComputer {
    fn init(program: std::path::PathBuf, inputs: Vec<isize>) -> Self {
        let raw_data = std::fs::read_to_string(program).unwrap();
        let mut memory: Vec<isize> =
            raw_data.rsplit(",").map(|opcode| opcode.trim().parse::<isize>().unwrap()).collect();
        memory.reverse();
        Self {
            ip: 0,
            memory_backup: memory.clone(),
            memory,
            halted: false,
            input_buffer: inputs,
            output_buffer: Vec::new(),
        }
    }

    #[inline]
    fn step(&mut self, amount: usize) {
        self.ip += amount;
    }

    #[inline]
    fn load(&self, addr: isize) -> isize {
        self.memory[addr as usize]
    }

    #[inline]
    fn save(&mut self, addr: isize, contents: isize) {
        self.memory[addr as usize] = contents;
    }

    fn debug_print(&self) {
        println!(
            "================================================================================"
        );
        println!("instruction pointer: {}", self.ip);
        println!("input buffer: {:?}\noutput buffer: {:?}", self.input_buffer, self.output_buffer);
        println!("system halted: {}", self.halted);
        println!(
            "================================================================================"
        );
        println!("{:?}", self.memory);
        println!(
            "================================================================================"
        );
    }

    fn process(&mut self) {
        let instruction = self.memory[self.ip] % 100;
        let modes = modes((self.memory[self.ip] / 100) as usize);

        match instruction {
            // add
            1 => {
                let a = match modes[0] {
                    0 => self.memory[self.memory[self.ip + 1] as usize],
                    1 => self.memory[self.ip + 1],
                    _ => panic!("uh oh"),
                };
                let b = match modes[0] {
                    0 => self.memory[self.memory[self.ip + 2] as usize],
                    1 => self.memory[self.ip + 2],
                    _ => panic!("uh oh"),
                };
                let dest = self.memory[self.ip + 3] as usize;
                self.memory[dest] = a + b;
                self.ip += 4;
            }
            // mul
            2 => {
                let a = match modes[0] {
                    0 => self.memory[self.memory[self.ip + 1] as usize],
                    1 => self.memory[self.ip + 1],
                    _ => panic!("uh oh"),
                };
                let b = match modes[0] {
                    0 => self.memory[self.memory[self.ip + 2] as usize],
                    1 => self.memory[self.ip + 2],
                    _ => panic!("uh oh"),
                };
                let dest = self.memory[self.ip + 3] as usize;
                self.memory[dest] = a * b;
                self.ip += 4;
            }
            // inp
            3 => {
                let input = match self.input_buffer.pop() {
                    Some(input) => input,
                    None => panic!("input buffer is empty!"),
                };
                let dest = self.memory[self.ip + 1] as usize;
                self.memory[dest] = input;
                self.ip += 2;
            }
            // str
            4 => {
                let source = self.memory[self.ip + 1] as usize;
                self.output_buffer.push(self.memory[source]);
                self.ip += 2;
            }
            // hlt
            99 => {
                self.halted = true;
            }
            // unrecognized opcode
            _ => {
                eprintln!("!!! Unrecognized opcode at instruction: {}", self.ip);
                self.halted = true;
                self.debug_print();
            }
        }
    }

    fn reset(&mut self) {
        self.ip = 0;
        self.memory = self.memory_backup.clone();
        self.halted = false;
    }
}

fn modes(modes: usize) -> [usize; 3] {
    let mut modes = modes.clone();
    let mut modes_arr = [0; 3];
    modes_arr[0] = modes % 10;
    modes /= 10;
    modes_arr[1] = modes % 10;
    modes /= 10;
    modes_arr[2] = modes % 10;
    modes_arr
}

pub fn part_one() {
    let timer = std::time::Instant::now();

    let mut comp = IntCodeComputer::init(std::path::PathBuf::from("./inputs/day_five"), vec![1]);
    loop {
        comp.debug_print();
        comp.process();
        if comp.halted {
            break;
        }
    }

    println!("execution time: {}ms", timer.elapsed().as_millis());
}

pub fn part_two() {
    let timer = std::time::Instant::now();

    //code here

    println!("execution time: {}ms", timer.elapsed().as_millis());
}

#[cfg(test)]
mod day_five {
    use super::*;

    #[test]
    fn day_two_part_one() {
        let mut comp =
            IntCodeComputer::init(std::path::PathBuf::from("./inputs/day_two"), Vec::new());

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
        let mut comp =
            IntCodeComputer::init(std::path::PathBuf::from("./inputs/day_two"), Vec::new());

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
}
