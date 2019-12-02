#[derive(Debug)]
struct IntCodeComputer {
    memory: Vec<usize>,
    memory_backup: Vec<usize>,
    ip: usize,
    halt: bool,
}

impl IntCodeComputer {
    fn init() -> Self {
        let raw_data = std::fs::read_to_string("./inputs/day_two").unwrap();
        let mut memory: Vec<usize> =
            raw_data.rsplit(",").map(|opcode| opcode.trim().parse::<usize>().unwrap()).collect();
        memory.reverse();
        Self { ip: 0, memory_backup: memory.clone(), memory, halt: false }
    }

    #[inline]
    fn step(&mut self) {
        self.ip += 4;
    }

    #[inline]
    fn get_params(&self) -> [usize; 3] {
        [self.memory[self.ip + 1], self.memory[self.ip + 2], self.memory[self.ip + 3]]
    }

    #[inline]
    fn load(&self, addr: usize) -> usize {
        self.memory[addr]
    }

    #[inline]
    fn save(&mut self, addr: usize, contents: usize) {
        self.memory[addr] = contents;
    }

    fn process(&mut self) {
        match self.memory[self.ip] {
            1 => {
                let p = self.get_params();
                let (a, b) = (self.load(p[0]), self.load(p[1]));
                self.save(p[2], a + b)
            }
            2 => {
                let p = self.get_params();
                let (a, b) = (self.load(p[0]), self.load(p[1]));
                self.save(p[2], a * b)
            }
            99 => {
                println!("Program Halted at instruction: {}", self.ip);
                println!("memory dump: {:?}", self);
                self.halt = true;
            }
            _ => {
                eprintln!("Unrecognized opcode at instruction: {}", self.ip);
                eprintln!("memory dump: {:?}", self);
                self.halt = false;
            }
        }
    }

    fn reset(&mut self) {
        self.ip = 0;
        self.memory = self.memory_backup.clone();
        self.halt = false;
    }
}

pub fn part_one() {
    let mut comp = IntCodeComputer::init();
    comp.memory[1] = 12;
    comp.memory[2] = 2;
    println!("initial state: {:?}", &comp);
    loop {
        comp.process();
        comp.step();
        if comp.halt {
            std::process::exit(0);
        }
    }
}

pub fn part_two() {
    let mut comp = IntCodeComputer::init();
    for noun in 0..99 {
        for verb in 0..99 {
            comp.memory[1] = noun;
            comp.memory[2] = verb;
            loop {
                comp.process();
                comp.step();
                if comp.halt {
                    break;
                }
            }
            if comp.memory[0] == 19690720 {
                println!("{:?}", 100 * noun + verb);
                std::process::exit(0);
            }
            comp.reset();
        }
    }
}
