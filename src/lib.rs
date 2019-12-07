// intcode computer tests
pub mod tests;

// intcode computer struct
#[derive(Debug)]
pub struct IntCodeComputer {
    pub memory: Vec<isize>,
    pub memory_backup: Vec<isize>,
    pub ip: usize,
    pub halted: bool,
    pub output_buffer: Vec<isize>,
    pub input_buffer: Vec<isize>,
}

#[allow(dead_code)]
impl IntCodeComputer {
    pub fn init(program: std::path::PathBuf, inputs: Vec<isize>) -> Self {
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
    pub fn step(&mut self, amount: usize) {
        self.ip += amount;
    }

    #[inline]
    pub fn load(&self, addr: isize) -> isize {
        self.memory[addr as usize]
    }

    #[inline]
    pub fn save(&mut self, addr: isize, contents: isize) {
        self.memory[addr as usize] = contents;
    }

    pub fn debug_print(&self) {
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

    pub fn modes(modes: usize) -> [usize; 3] {
        let mut modes = modes.clone();
        let mut modes_arr = [0; 3];
        modes_arr[0] = modes % 10;
        modes /= 10;
        modes_arr[1] = modes % 10;
        modes /= 10;
        modes_arr[2] = modes % 10;
        modes_arr
    }

    pub fn process(&mut self) {
        let instruction = self.memory[self.ip] % 100;
        let modes = IntCodeComputer::modes((self.memory[self.ip] / 100) as usize);

        match instruction {
            // add
            1 => {
                let a = match modes[0] {
                    0 => self.memory[self.memory[self.ip + 1] as usize],
                    1 => self.memory[self.ip + 1],
                    _ => panic!("invalid parameter mode!"),
                };
                let b = match modes[1] {
                    0 => self.memory[self.memory[self.ip + 2] as usize],
                    1 => self.memory[self.ip + 2],
                    _ => panic!("invalid parameter mode!"),
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
                    _ => panic!("invalid parameter mode!"),
                };
                let b = match modes[1] {
                    0 => self.memory[self.memory[self.ip + 2] as usize],
                    1 => self.memory[self.ip + 2],
                    _ => panic!("invalid parameter mode!"),
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
            // jt
            5 => {
                let cmp = match modes[0] {
                    0 => self.memory[self.memory[self.ip + 1] as usize],
                    1 => self.memory[self.ip + 1],
                    _ => panic!("invalid parameter mode!"),
                };
                let dest = match modes[1] {
                    0 => self.memory[self.memory[self.ip + 2] as usize] as usize,
                    1 => self.memory[self.ip + 2] as usize,
                    _ => panic!("invalid parameter mode!"),
                };

                if cmp != 0 {
                    self.ip = dest;
                } else {
                    self.ip += 3;
                }
            }
            // jf
            6 => {
                let cmp = match modes[0] {
                    0 => self.memory[self.memory[self.ip + 1] as usize],
                    1 => self.memory[self.ip + 1],
                    _ => panic!("invalid parameter mode!"),
                };
                let dest = match modes[1] {
                    0 => self.memory[self.memory[self.ip + 2] as usize] as usize,
                    1 => self.memory[self.ip + 2] as usize,
                    _ => panic!("invalid parameter mode!"),
                };

                if cmp == 0 {
                    self.ip = dest;
                } else {
                    self.ip += 3;
                }
            }
            // lt
            7 => {
                let cmp_one = match modes[0] {
                    0 => self.memory[self.memory[self.ip + 1] as usize],
                    1 => self.memory[self.ip + 1],
                    _ => panic!("invalid parameter mode!"),
                };
                let cmp_two = match modes[1] {
                    0 => self.memory[self.memory[self.ip + 2] as usize],
                    1 => self.memory[self.ip + 2],
                    _ => panic!("invalid parameter mode!"),
                };
                let dest = self.memory[self.ip + 3] as usize;
                if cmp_one < cmp_two {
                    self.memory[dest] = 1;
                } else {
                    self.memory[dest] = 0;
                }
                self.ip += 4;
            }
            // eq
            8 => {
                let cmp_one = match modes[0] {
                    0 => self.memory[self.memory[self.ip + 1] as usize],
                    1 => self.memory[self.ip + 1],
                    _ => panic!("invalid parameter mode!"),
                };
                let cmp_two = match modes[1] {
                    0 => self.memory[self.memory[self.ip + 2] as usize],
                    1 => self.memory[self.ip + 2],
                    _ => panic!("invalid parameter mode!"),
                };
                let dest = self.memory[self.ip + 3] as usize;
                if cmp_one == cmp_two {
                    self.memory[dest] = 1;
                } else {
                    self.memory[dest] = 0;
                }
                self.ip += 4;
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

    pub fn reset(&mut self) {
        self.ip = 0;
        self.memory = self.memory_backup.clone();
        self.halted = false;
    }
}
