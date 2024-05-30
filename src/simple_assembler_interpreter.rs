use std::collections::HashMap;

/// https://www.codewars.com/kata/58e24788e24ddee28e000053/train/rust
pub fn simple_assembler(program: Vec<&str>) -> HashMap<String, i64> {
    let mut machine = Machine::new();
    machine.execute(program);

    machine.registers
}

struct Machine {
    registers: HashMap<String, i64>,
    instruction_pointer: i64,
}

impl Machine {
    fn new() -> Machine {
        Machine {
            registers: HashMap::new(),
            instruction_pointer: 0,
        }
    }

    fn get(&self, register: String) -> i64 {
        *self.registers.get(&register).unwrap_or(&0)
    }

    fn store(&mut self, register: String, value: i64) {
        self.registers.insert(register, value);
    }

    fn execute(&mut self, program: Vec<&str>) {
        let instructions = parse_instructions(program);

        while self.instruction_pointer >= 0 && self.instruction_pointer < instructions.len() as i64 {
            let instruction = &instructions[self.instruction_pointer as usize];

            instruction.execute(self);
            self.instruction_pointer += 1;
        }
    }
}

fn parse_instructions(program: Vec<&str>) -> Vec<Box<dyn Instruction>> {
    program.iter()
        .map(|instruction_str| instruction_str.to_string())
        .map(parse_instruction)
        .collect()
}

fn parse_instruction(instruction_str: String) -> Box<dyn Instruction> {
    let split: Vec<&str> = instruction_str.split(' ').collect();

    match *split.first().unwrap() {
        "mov" => {
            let x = split.get(1).unwrap().to_string();
            let y = split.get(2).unwrap().to_string();

            Box::new(Mov {
                x,
                y,
            })
        }
        "inc" => {
            let x = split.get(1).unwrap().to_string();

            Box::new(Inc {
                x,
            })
        }
        "dec" => {
            let x = split.get(1).unwrap().to_string();

            Box::new(Dec {
                x,
            })
        }
        "jnz" => {
            let x = split.get(1).unwrap().to_string();
            let y = split.get(2).unwrap().to_string();

            Box::new(Jnz {
                x,
                y,
            })
        }
        _ => panic!("Unknown instruction: {}", instruction_str),
    }
}

trait Instruction {
    fn execute(&self, machine: &mut Machine);
}

struct Mov {
    x: String,
    y: String,
}

impl Instruction for Mov {
    fn execute(&self, machine: &mut Machine) {
        let y_value = self.y.parse()
            .unwrap_or_else(|_| machine.get(self.y.clone()));

        machine.store(self.x.clone(), y_value);
    }
}

struct Inc {
    x: String,
}

impl Instruction for Inc {
    fn execute(&self, machine: &mut Machine) {
        let x_value = machine.get(self.x.clone());
        machine.store(self.x.clone(), x_value + 1);
    }
}

struct Dec {
    x: String,
}

impl Instruction for Dec {
    fn execute(&self, machine: &mut Machine) {
        let x_value = machine.get(self.x.clone());
        machine.store(self.x.clone(), x_value - 1);
    }
}

struct Jnz {
    x: String,
    y: String,
}

impl Instruction for Jnz {
    fn execute(&self, machine: &mut Machine) {
        let x_value = self.x.parse()
            .unwrap_or_else(|_| machine.get(self.x.clone()));

        if x_value != 0 {
            let y_value = self.y.parse()
                .unwrap_or_else(|_| machine.get(self.y.clone()));

            machine.instruction_pointer += y_value - 1;
        }
    }
}
