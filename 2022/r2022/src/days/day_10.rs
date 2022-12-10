

trait Opcode {
    fn tick(&mut self, registers: &mut [i32]) -> bool;
}

struct Addx {
    val: i32,
    ticks: u8
}

impl Addx {
    fn new(val: i32) -> Addx {
        Addx {val, ticks: 0}
    }
}

impl Opcode for Addx {
    fn tick(&mut self, registers: &mut[i32]) -> bool {
        self.ticks += 1;

        if self.ticks >= 2 {
            registers[0] += self.val;
            return true;
        }

        false
    }
}

struct Noop {

}

impl Opcode for Noop {
    fn tick(&mut self, _registers: &mut[i32]) -> bool {
        true
    }
}

struct Vm {
    registers: [i32; 1],
    instructions: Vec<Box<dyn Opcode>>,
    inst_p: usize
}

impl Vm {
    fn tick(&mut self) -> bool {
        let instruction = &mut self.instructions[self.inst_p];
        if instruction.tick(&mut self.registers) {
            self.inst_p += 1;
            
            if self.inst_p >= self.instructions.len() {
                return true;
            }
        }

        false
    }
}

fn parse_input(inp: &str) -> Vec<Box<dyn Opcode>> {
    let mut res: Vec<Box<dyn Opcode>> = Vec::new();

    for line in inp.split("\n") {
        if line.is_empty() {
            continue;
        }

        let split: Vec<&str> = line.split(" ").collect();

        match split[0] {
            "addx" => res.push(Box::new(Addx::new(split[1].parse::<i32>().unwrap()))),
            "noop" => res.push(Box::new(Noop {})),
            _ => panic!("unrecognized opcode \"{}\"", split[0])
        }
    }

    res
}

pub fn run(inp: &str) {
    let opcodes = parse_input(inp);
    let mut vm = Vm {registers: [1; 1], instructions: opcodes, inst_p: 0};
    let mut v: Vec<i32> = Vec::new();

    loop {
        v.push(vm.registers[0]);
        let br = vm.tick();

        if br {break;}
    }

    for x in &v {
        println!("{}", x);
    }
    println!(
        "20: {} ({}), 60: {}, 100: {}, 140: {}, 180: {}, 220: {} ({})",
        v[19] * 20,
        v[19],
        v[59] * 60,
        v[99] * 100,
        v[139] * 140,
        v[179] * 180,
        v[219] * 220,
        v[219]
    );
    println!("sum: {}",
        v[19] * 20
        + v[59] * 60
        + v[99] * 100
        + v[139] * 140
        + v[179]* 180
        + v[219] * 220
    );

}