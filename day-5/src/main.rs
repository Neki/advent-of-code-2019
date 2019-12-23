use std::error;
use std::fs::File;
use std::io::{stdin, Read};
use std::num;
use std::convert::TryInto;
use std::ops::{Index, IndexMut};


fn main() {
    run_interpreter_on_file("input.txt");
}

// this lacks error handling, as in everywhere else in this file...
fn run_interpreter_on_file(path: &str) {
    let mut memory = Memory::from_file(path).unwrap();
    let mut interpreter = Interpreter::from_memory(&mut memory);
    interpreter.run();
}

#[derive(PartialEq, Debug)]
struct Memory(Vec<isize>);

impl Memory {
    fn from_string(input: &str) -> Result<Memory, num::ParseIntError> {
        let mut out: Vec<isize> = Vec::new();
        for s in input.trim().split(',') {
            let parsed = s.parse::<isize>()?;
            out.push(parsed);
        }
        Ok(Memory(out))
    }

    fn from_file(path: &str) -> Result<Memory, Box<dyn error::Error>> {
        let mut file = File::open(path)?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;
        match Memory::from_string(&buffer) {
            Ok(memory) => Ok(memory),
            Err(error) => Err(Box::new(error)),
        }
    }
}

impl Index<usize> for Memory {
    type Output = isize;

    fn index(&self, address: usize) -> &Self::Output {
        &self.0[address]
    }
}

impl IndexMut<usize> for Memory {
    fn index_mut(&mut self, address: usize) -> &mut Self::Output {
        &mut self.0[address]
    }
}

struct Interpreter<'a> {
    memory: &'a mut Memory,
    instruction_pointer: usize,
}

impl Interpreter<'_> {
    fn from_memory(memory: &mut Memory) -> Interpreter {
        Interpreter {
            memory,
            instruction_pointer: 0,
        }
    }

    // TODO out of bound checking and real error handling
    fn run(&mut self) -> Option<()> {
        loop {
            let instruction_def_int = self.memory[self.instruction_pointer];
            let instruction_def = parse_instruction_definition(instruction_def_int).unwrap();
            if instruction_def.opcode == Opcode::Halt {
                return Some(());
            }
            self.run_instruction(&instruction_def);
        }
    }

    fn run_instruction(&mut self, instruction_def: &InstructionDefinition) {
        match instruction_def.opcode {
            Opcode::Addition => {
                self.run_instruction_addition(&instruction_def.instruction_modes);
            }
            Opcode::Multiplication => {
                self.run_instruction_multiplication(&instruction_def.instruction_modes);
            }
            Opcode::Input => {
                self.run_instruction_input();
            }
            Opcode::Output => {
                self.run_instruction_output(&instruction_def.instruction_modes);
            }
            Opcode::JumpIfTrue => {
                self.run_instruction_jump_if_true(&instruction_def.instruction_modes);
            }
            Opcode::JumpIfFalse => {
                self.run_instruction_jump_if_false(&instruction_def.instruction_modes);
            }
            Opcode::LessThan => {
                self.run_instruction_less_than(&instruction_def.instruction_modes);
            }
            Opcode::Equals => {
                self.run_instruction_equals(&instruction_def.instruction_modes);
            }
            Opcode::Halt => (),
        }
    }

    fn run_instruction_addition(&mut self, instruction_modes: &[InstructionMode]) {
        let left_operand = self.read_value(
            self.memory[self.instruction_pointer + 1],
            &instruction_modes[0],
        );
        let right_operand = self.read_value(
            self.memory[self.instruction_pointer + 2],
            &instruction_modes[1],
        );
        let result = left_operand + right_operand;
        let result_address: usize = self.memory[self.instruction_pointer + 3].try_into().unwrap();
        self.memory[result_address] = result;
        self.instruction_pointer += 4
    }

    // TODO remove duplicated code (need function run_binary_instruction)
    fn run_instruction_multiplication(&mut self, instruction_modes: &[InstructionMode]) {
        let left_operand = self.read_value(
            self.memory[self.instruction_pointer + 1],
            &instruction_modes[0],
        );
        let right_operand = self.read_value(
            self.memory[self.instruction_pointer + 2],
            &instruction_modes[1],
        );
        let result = left_operand * right_operand;
        let result_address: usize = self.memory[self.instruction_pointer + 3].try_into().unwrap();
        self.memory[result_address] = result;
        self.instruction_pointer += 4
    }

    fn run_instruction_input(&mut self) {
        let mut input = String::new();
        println!("enter input (integer):");
        stdin().read_line(&mut input).unwrap();
        let parsed = input.trim().parse::<isize>().unwrap();
        let address: usize = self.memory[self.instruction_pointer + 1].try_into().unwrap();
        self.memory[address] = parsed;
        self.instruction_pointer += 2;
    }

    fn run_instruction_output(&mut self, instruction_modes: &[InstructionMode]) {
        let operand = self.read_value(
            self.memory[self.instruction_pointer + 1],
            &instruction_modes[0],
        );
        println!("output: {}", operand);
        self.instruction_pointer += 2;
    }

    fn run_instruction_jump_if_true(&mut self, instruction_modes: &[InstructionMode]) {
        let operand = self.read_value(
            self.memory[self.instruction_pointer + 1],
            &instruction_modes[0],
        );
        if operand != 0 {
            let jump_address: usize = self.read_value(self.memory[self.instruction_pointer + 2], &instruction_modes[1]).try_into().unwrap();
            self.instruction_pointer = jump_address;
        } else {
            self.instruction_pointer += 3;
        }
    }

    // TODO remove duplicated code
    fn run_instruction_jump_if_false(&mut self, instruction_modes: &[InstructionMode]) {
        let operand = self.read_value(
            self.memory[self.instruction_pointer + 1],
            &instruction_modes[0],
        );
        if operand == 0 {
            let jump_address: usize = self.read_value(self.memory[self.instruction_pointer + 2], &instruction_modes[1]).try_into().unwrap();
            self.instruction_pointer = jump_address;
        } else {
            self.instruction_pointer += 3;
        }
    }

    fn run_instruction_less_than(&mut self, instruction_modes: &[InstructionMode]) {
        let first_parameter = self.read_value(
            self.memory[self.instruction_pointer + 1],
            &instruction_modes[0],
        );
        let second_parameter = self.read_value(
            self.memory[self.instruction_pointer + 2],
            &instruction_modes[1],
        );
        let to_store = if first_parameter < second_parameter { 1 } else { 0 };
        let result_address: usize = self.memory[self.instruction_pointer + 3].try_into().unwrap();
        self.memory[result_address] = to_store;
        self.instruction_pointer += 4;
    }

    // TODO remove duplicated code
    fn run_instruction_equals(&mut self, instruction_modes: &[InstructionMode]) {
        let first_parameter = self.read_value(
            self.memory[self.instruction_pointer + 1],
            &instruction_modes[0],
        );
        let second_parameter = self.read_value(
            self.memory[self.instruction_pointer + 2],
            &instruction_modes[1],
        );
        let to_store = if first_parameter == second_parameter { 1 } else { 0 };
        let result_address: usize = self.memory[self.instruction_pointer + 3].try_into().unwrap();
        self.memory[result_address] = to_store;
        self.instruction_pointer += 4;
    }

    fn read_value(&self, parameter: isize, mode: &InstructionMode) -> isize {
        match mode {
            InstructionMode::Position => self.memory[parameter.try_into().unwrap()],
            InstructionMode::Immediate => parameter,
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
enum Opcode {
    Addition,
    Multiplication,
    Input,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    Halt,
}

#[derive(PartialEq, Debug, Clone)]
enum InstructionMode {
    Position,
    Immediate,
}

#[derive(PartialEq, Debug)]
struct InstructionDefinition {
    opcode: Opcode,
    instruction_modes: Vec<InstructionMode>,
}

fn parse_instruction_definition(instruction_code: isize) -> Option<InstructionDefinition> {
    let opcode = parse_opcode(instruction_code % 100).expect(format!("Can not parse opcode: {}", instruction_code).as_str());
    // TODO some validation? make sure length matches opcode?
    let instruction_modes = parse_instruction_modes(instruction_code / 100).expect(format!("Can not parse instruction modes: {}", instruction_code).as_str());
    Some(InstructionDefinition {
        opcode,
        instruction_modes,
    })
}

fn parse_opcode(opcode_int: isize) -> Option<Opcode> {
    match opcode_int {
        1 => Some(Opcode::Addition),
        2 => Some(Opcode::Multiplication),
        3 => Some(Opcode::Input),
        4 => Some(Opcode::Output),
        5 => Some(Opcode::JumpIfTrue),
        6 => Some(Opcode::JumpIfFalse),
        7 => Some(Opcode::LessThan),
        8 => Some(Opcode::Equals),
        99 => Some(Opcode::Halt),
        _ => None,
    }
}

fn parse_instruction_modes(instruction_modes_int: isize) -> Option<Vec<InstructionMode>> {
    let mut instruction_modes = Vec::new();
    let mut instruction_modes_int = instruction_modes_int;
    loop {
        instruction_modes.push(match instruction_modes_int % 10 {
            0 => InstructionMode::Position,
            1 => InstructionMode::Immediate,
            _ => return None,
        });
        if instruction_modes_int < 10 {
            // always return a vector of size at least 2 to simplify
            if instruction_modes.len() == 1 {
                instruction_modes.push(InstructionMode::Position);
            }
            return Some(instruction_modes);
        }
        instruction_modes_int /= 10;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interpreter_run_complex_example() {
        assert_interpreter_run(
            "1,9,10,3,2,3,11,0,99,30,40,50",
            "3500,9,10,70,2,3,11,0,99,30,40,50",
        );
    }

    #[test]
    fn test_interpreter_run_simple_examples() {
        assert_interpreter_run("1,0,0,0,99", "2,0,0,0,99");
        assert_interpreter_run("2,3,0,3,99", "2,3,0,6,99");
        assert_interpreter_run("2,4,4,5,99,0", "2,4,4,5,99,9801");
        assert_interpreter_run("1,1,1,4,99,5,6,0,99", "30,1,1,4,2,5,6,0,99");
    }

    #[test]
    fn test_interpreter_run_immediate_mode() {
        assert_interpreter_run("1002,4,3,4,33", "1002,4,3,4,99");
    }

    fn assert_interpreter_run(starting_memory: &str, expected_memory: &str) {
        let mut memory = Memory::from_string(starting_memory).unwrap();
        let mut interpreter = Interpreter::from_memory(&mut memory);
        let output = interpreter.run();
        assert!(output.is_some());
        let expected_memory = Memory::from_string(expected_memory).unwrap();
        assert_eq!(memory, expected_memory);
    }

    #[test]
    fn test_parse_instruction_definition() {
        assert_eq!(
            Some(InstructionDefinition {
                opcode: Opcode::Multiplication,
                instruction_modes: Vec::from(
                    [InstructionMode::Position, InstructionMode::Immediate].to_vec()
                )
            }),
            parse_instruction_definition(1002)
        );

        assert_eq!(
            Some(InstructionDefinition {
                opcode: Opcode::Input,
                instruction_modes: Vec::from(
                    [InstructionMode::Position, InstructionMode::Position].to_vec()
                )
            }),
            parse_instruction_definition(3)
        );
    }
}
