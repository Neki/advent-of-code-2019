use std::error;
use std::fs::File;
use std::num;
use std::io::{Read};
use std::ops::{Index, IndexMut};

fn main() -> Result<(), Box<dyn error::Error>> {
    part_1()?;
    part_2()?;
    Ok(())
}

fn part_1() -> Result<(), Box<dyn error::Error>> {
    let output = run_interpreter_on_file("input.txt", 12, 2);
    println!("{}", output);
    Ok(())
}

fn part_2() -> Result<(), Box<dyn error::Error>> {
    for noun in 0..99 {
        for verb in 0..99 {
            let output = run_interpreter_on_file("input.txt", noun, verb);
            if output == 19_690_720  {
                let solution = 100 * noun + verb;
                println!("{}", solution);
                return Ok(());
            }
        }
    }
    panic!("no solution found to part 2");
}

// this lacks error handling, as in everywhere else in this file...
fn run_interpreter_on_file(path: &str, noun: usize, verb: usize) -> usize {
    let mut memory = Memory::from_file(path).unwrap();
    memory.set_inputs(noun, verb);
    let mut interpreter = Interpreter::from_memory(&mut memory);
    interpreter.run()
}

#[derive(PartialEq, Debug)]
struct Memory(Vec<usize>);

impl Memory {
    fn from_string(input: &str) -> Result<Memory, num::ParseIntError> {
        let mut out: Vec<usize> = Vec::new();
        for s in input.trim().split(',') {
            let parsed = s.parse::<usize>()?;
            out.push(parsed);
        }
        Ok(Memory(out))

    }

    fn set_inputs(&mut self, noun: usize, verb: usize) {
        self[1] = noun;
        self[2] = verb;
    }

    fn from_file(path: &str) -> Result<Memory, Box<dyn error::Error>> {
        let mut file = File::open(path)?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;
        match Memory::from_string(&buffer) {
            Ok(memory) => Ok(memory),
            Err(error) => Err(Box::new(error))
        }
    }

}

impl Index<usize> for Memory {
    type Output = usize;

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
    instruction_pointer: usize
}

impl Interpreter<'_> {
    fn from_memory(memory: &mut Memory) -> Interpreter {
        Interpreter{
            memory,
            instruction_pointer: 0
        }
    }

    // TODO out of bound checking and real error handling
    fn run(&mut self) -> usize {
        loop {
            let opcode_int = self.memory[self.instruction_pointer];
            if let Some(opcode) = parse_opcode(opcode_int) {
                if opcode == Opcode::Halt {
                    return self.memory[0];
                }
                self.run_instruction(&opcode);
            } else {
                panic!("invalid opcode")
            }
        }
    }

    fn run_instruction(&mut self, opcode: &Opcode) {
        match opcode {
            Opcode::Addition => {
                self.run_instruction_addition();
            }
            Opcode::Multiplication => {
                self.run_instruction_multiplication();
            }
            Opcode::Halt => ()

        }
    }

    fn run_instruction_addition(&mut self) {
        let left_operand = self.memory[self.memory[self.instruction_pointer + 1]];
        let right_operand = self.memory[self.memory[self.instruction_pointer + 2]];
        let result = left_operand + right_operand;
        let result_address = self.memory[self.instruction_pointer + 3];
        self.memory[result_address] = result;
        self.instruction_pointer += 4
    }

    // TODO remove duplicated code (need function run_binary_instruction)
    fn run_instruction_multiplication(&mut self) {
        let left_operand = self.memory[self.memory[self.instruction_pointer + 1]];
        let right_operand = self.memory[self.memory[self.instruction_pointer + 2]];
        let result = left_operand * right_operand;
        let result_address = self.memory[self.instruction_pointer + 3];
        self.memory[result_address] = result;
        self.instruction_pointer += 4
    }

}

#[derive(PartialEq)]
enum Opcode {
    Addition,
    Multiplication,
    Halt
}

fn parse_opcode(opcode_int: usize) -> Option<Opcode> {
    match opcode_int {
        1 => Some(Opcode::Addition),
        2 => Some(Opcode::Multiplication),
        99 => Some(Opcode::Halt),
        _ => None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interpreter_run_complex_example() {
        assert_interpreter_run("1,9,10,3,2,3,11,0,99,30,40,50", "3500,9,10,70,2,3,11,0,99,30,40,50", 3500);
    }

    #[test]
    fn test_interpreter_run_simple_examples() {
        assert_interpreter_run("1,0,0,0,99", "2,0,0,0,99", 2);
        assert_interpreter_run("2,3,0,3,99", "2,3,0,6,99", 2);
        assert_interpreter_run("2,4,4,5,99,0", "2,4,4,5,99,9801", 2);
        assert_interpreter_run("1,1,1,4,99,5,6,0,99", "30,1,1,4,2,5,6,0,99", 30);
    }

    fn assert_interpreter_run(starting_memory: &str, expected_memory: &str, expected_output: usize) {
        let mut memory = Memory::from_string(starting_memory).unwrap();
        let mut interpreter = Interpreter::from_memory(&mut memory);
        let result = interpreter.run();
        assert_eq!(result, expected_output);
        let expected_memory = Memory::from_string(expected_memory).unwrap();
        assert_eq!(memory, expected_memory);
    }

}
