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

impl IntCodeComputer {
    /// initializes the computer and loads a program to memory
    pub fn init(program: std::path::PathBuf, inputs: Vec<isize>) -> Self {
        let raw_data = std::fs::read_to_string(program).unwrap();
        let memory: Vec<isize> =
            raw_data.split(",").map(|opcode| opcode.trim().parse::<isize>().unwrap()).collect();
        Self {
            ip: 0,
            memory_backup: memory.clone(),
            memory,
            halted: false,
            input_buffer: inputs,
            output_buffer: Vec::new(),
        }
    }

    /// prints computer information: instruction pointer, buffers, system halt state, and memory dump
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

    /// converts an opcodes first three digits into usable parameter mode information
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

    /// loads parameter from memory given offset from instruction pointer and mode
    #[inline]
    fn load_parameter(&self, mode: usize, offset: usize) -> isize {
        match mode {
            0 => self.memory[self.memory[self.ip + offset] as usize],
            1 => self.memory[self.ip + offset],
            _ => panic!("invalid parameter mode!"),
        }
    }

    /// loads input from input buffer
    #[inline]
    fn take_input(&mut self) -> isize {
        match self.input_buffer.pop() {
            Some(input) => input,
            None => panic!("input buffer is empty!"),
        }
    }

    /// writes output to output buffer
    #[inline]
    fn write_output(&mut self, data: isize) {
        self.output_buffer.push(data);
    }

    /// step amount forward in memory
    #[inline]
    fn step(&mut self, amount: usize) {
        self.ip += amount;
    }

    /// jump to address in memory
    #[inline]
    fn jump(&mut self, address: usize) {
        self.ip = address
    }

    /// set given address to given contents
    pub fn set_addr(&mut self, address: usize, contents: isize) {
        self.memory[address] = contents;
    }

    /// process the machine code
    pub fn process(&mut self) {
        let instruction = self.memory[self.ip] % 100;
        let modes = IntCodeComputer::modes((self.memory[self.ip] / 100) as usize);

        match instruction {
            // add
            1 => {
                let a = self.load_parameter(modes[0], 1);
                let b = self.load_parameter(modes[1], 2);
                let dest = self.load_parameter(1, 3) as usize;
                self.memory[dest] = a + b;
                self.step(4);
            }
            // mul
            2 => {
                let a = self.load_parameter(modes[0], 1);
                let b = self.load_parameter(modes[1], 2);
                let dest = self.load_parameter(1, 3) as usize;
                self.memory[dest] = a * b;
                self.step(4);
            }
            // inp
            3 => {
                let input = self.take_input();
                let dest = self.load_parameter(1, 1) as usize;
                self.memory[dest] = input;
                self.step(2);
            }
            // out
            4 => {
                let source = self.load_parameter(1, 1) as usize;
                self.write_output(self.memory[source]);
                self.step(2);
            }
            // jt
            5 => {
                let cmp = self.load_parameter(modes[0], 1);
                let dest = self.load_parameter(modes[1], 2) as usize;
                if cmp != 0 {
                    self.jump(dest);
                } else {
                    self.step(3);
                }
            }
            // jf
            6 => {
                let cmp = self.load_parameter(modes[0], 1);
                let dest = self.load_parameter(modes[1], 2) as usize;
                if cmp == 0 {
                    self.jump(dest);
                } else {
                    self.step(3);
                }
            }
            // lt
            7 => {
                let cmp_one = self.load_parameter(modes[0], 1);
                let cmp_two = self.load_parameter(modes[1], 2);
                let dest = self.load_parameter(1, 3) as usize;
                if cmp_one < cmp_two {
                    self.set_addr(dest, 1);
                } else {
                    self.set_addr(dest, 0);
                }
                self.step(4);
            }
            // eq
            8 => {
                let cmp_one = self.load_parameter(modes[0], 1);
                let cmp_two = self.load_parameter(modes[1], 2);
                let dest = self.load_parameter(1, 3) as usize;
                if cmp_one == cmp_two {
                    self.set_addr(dest, 1);
                } else {
                    self.set_addr(dest, 0);
                }
                self.step(4);
            }
            // hlt
            99 => {
                self.halted = true;
            }

            // unrecognized opcode
            _ => {
                println!("================================================================================");
                eprintln!("!!! Unrecognized opcode at instruction: {} !!!", self.ip);
                self.halted = true;
                self.debug_print();
            }
        }
    }

    /// resets the computer back to it's initial state having just loaded the program to memory
    pub fn reset(&mut self) {
        self.ip = 0;
        self.memory = self.memory_backup.clone();
        self.halted = false;
    }
}
