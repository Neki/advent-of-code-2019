use std::collections::VecDeque;
use std::convert::TryInto;
use std::error;
use std::fs::File;
use std::io;
use std::io::Read;
use std::num;
use std::ops::{Index, IndexMut};

type Address = usize;
type Value = isize;

#[derive(PartialEq, Debug, Clone)]
struct Memory {
    values: Vec<Value>,
    relative_base: Value,
}

impl Memory {
    fn from_string(input: &str) -> Result<Memory, num::ParseIntError> {
        // assume we'll need, on average, one memory cell per 3 characters in the input
        let mut out: Vec<Value> = Vec::with_capacity(input.len() / 3);
        for s in input.trim().split(',') {
            let parsed = s.parse::<Value>()?;
            out.push(parsed);
        }
        Ok(Memory {
            values: out,
            relative_base: 0,
        })
    }

    fn read_value(&self, parameter: Value, mode: &InstructionMode) -> Value {
        match mode {
            InstructionMode::Position => self[parameter.try_into().unwrap()],
            InstructionMode::Immediate => parameter,
            InstructionMode::Relative => self[(parameter + self.relative_base as Value) as Address],
        }
    }

    // Yes, address is a Value (since it's read from memory)
    // What's important is that its an Address after taking into account the relative base
    fn write_value(&mut self, address: Value, value: Value, mode: &InstructionMode) {
        match mode {
            InstructionMode::Position => self[address.try_into().unwrap()] = value,
            InstructionMode::Immediate => panic!("attempted to write a value using immediate mode"),
            InstructionMode::Relative => {
                let relative_base = self.relative_base;
                self[(address + relative_base).try_into().unwrap()] = value
            }
        }
    }

    fn adjust_relative_base(&mut self, relative_base: Value) {
        self.relative_base += relative_base;
    }
}

impl Index<Address> for Memory {
    type Output = Value;

    fn index(&self, address: Address) -> &Self::Output {
        if address > self.values.len() {
            return &0;
        }
        &self.values[address]
    }
}

impl IndexMut<Address> for Memory {
    fn index_mut(&mut self, address: Address) -> &mut Self::Output {
        if address >= self.values.len() {
            self.values.resize(address + 1, 0);
        }
        &mut self.values[address]
    }
}

#[derive(Clone)]
pub struct Interpreter {
    memory: Memory,
    instruction_pointer: Address,
    input_queue: VecDeque<Value>,
}

#[derive(Debug)]
enum StepResult {
    NextInstruction,
    SetInstructionPointerTo(Address),
    ReadInput,
    ProduceOutput(Value),
    Halt,
}

#[non_exhaustive]
#[derive(Debug, PartialEq)]
pub enum IOAction {
    ReadInput,
    ProduceOutput(Value),
    Halt,
}

#[derive(Debug)]
pub struct ExecutionError {
    description: String,
    position: Address,
}

impl Interpreter {
    pub fn queue_input(&mut self, input: Value) {
        self.input_queue.push_back(input);
    }

    pub fn from_code(code: &str) -> Result<Self, num::ParseIntError> {
        let memory = Memory::from_string(code)?;
        Ok(Self::from_memory(memory))
    }

    pub fn from_file(filename: &str) -> Result<Self, Box<dyn error::Error>> {
        let mut file = File::open(filename)?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;
        match Self::from_code(&buffer) {
            Ok(x) => Ok(x),
            Err(err) => Err(Box::new(err)),
        }
    }

    fn from_memory(memory: Memory) -> Interpreter {
        Interpreter {
            memory,
            instruction_pointer: 0,
            input_queue: VecDeque::new(),
        }
    }

    pub fn run_until_block(&mut self) -> Result<IOAction, ExecutionError> {
        loop {
            let instruction_def_int = self.memory[self.instruction_pointer];
            // TODO remove unwrap
            let instruction_def = parse_instruction_definition(instruction_def_int).unwrap();
            // println!("{:?}", instruction_def);
            let step_result = self.run_instruction(&instruction_def);
            // println!("{:?}", step_result);

            // Advance instruction pointer
            match step_result {
                StepResult::SetInstructionPointerTo(jump_address) => {
                    self.instruction_pointer = jump_address
                }
                // The input instruction will return either ReadInput (= "WouldBlock") when not enough inputs are
                // available in the buffer, or NextInstruction if input is available.
                // In the first case, the instruction pointer should still point at the Input
                // instruction, so that on resume the instruction can try to read input again.
                StepResult::ReadInput => {}
                _ => self.instruction_pointer += instruction_length(instruction_def.opcode),
            }

            // Handle IO actions
            match step_result {
                StepResult::NextInstruction => {}
                StepResult::Halt => return Ok(IOAction::Halt),
                StepResult::SetInstructionPointerTo(_) => {}
                StepResult::ReadInput => return Ok(IOAction::ReadInput),
                StepResult::ProduceOutput(output) => return Ok(IOAction::ProduceOutput(output)),
            }
        }
    }

    pub fn run_interactively(&mut self) -> Result<(), ExecutionError> {
        loop {
            let io_action = self.run_until_block()?;
            match io_action {
                IOAction::Halt => return Ok(()),
                IOAction::ProduceOutput(x) => println!("output: {}", x),
                IOAction::ReadInput => {
                    println!("enter input: ");
                    let mut input = String::new();
                    io::stdin()
                        .read_line(&mut input)
                        .expect("error: unable to read user input");
                    let input_int = input
                        .parse::<Value>()
                        .expect("error: unable to parse user input as integer");
                    self.queue_input(input_int);
                }
            }
        }
    }

    fn run_instruction(&mut self, instruction_def: &InstructionDefinition) -> StepResult {
        match instruction_def.opcode {
            Opcode::Addition => self.run_instruction_addition(&instruction_def.instruction_modes),
            Opcode::Multiplication => {
                self.run_instruction_multiplication(&instruction_def.instruction_modes)
            }
            Opcode::Input => self.run_instruction_input(&instruction_def.instruction_modes),
            Opcode::Output => self.run_instruction_output(&instruction_def.instruction_modes),
            Opcode::JumpIfTrue => {
                self.run_instruction_jump_if_true(&instruction_def.instruction_modes)
            }
            Opcode::JumpIfFalse => {
                self.run_instruction_jump_if_false(&instruction_def.instruction_modes)
            }
            Opcode::LessThan => self.run_instruction_less_than(&instruction_def.instruction_modes),
            Opcode::Equals => self.run_instruction_equals(&instruction_def.instruction_modes),
            Opcode::AdjustRelativeBase => {
                self.run_instruction_adjust_relative_base(&instruction_def.instruction_modes)
            }
            Opcode::Halt => StepResult::Halt,
        }
    }

    fn run_instruction_addition(&mut self, instruction_modes: &[InstructionMode]) -> StepResult {
        let left_operand = self.memory.read_value(
            self.memory[self.instruction_pointer + 1],
            &instruction_modes[0],
        );
        let right_operand = self.memory.read_value(
            self.memory[self.instruction_pointer + 2],
            &instruction_modes[1],
        );
        let result = left_operand + right_operand;
        self.memory.write_value(
            self.memory[self.instruction_pointer + 3],
            result,
            &instruction_modes[2],
        );
        StepResult::NextInstruction
    }

    // TODO remove duplicated code (need function run_binary_instruction)
    fn run_instruction_multiplication(
        &mut self,
        instruction_modes: &[InstructionMode],
    ) -> StepResult {
        let left_operand = self.memory.read_value(
            self.memory[self.instruction_pointer + 1],
            &instruction_modes[0],
        );
        let right_operand = self.memory.read_value(
            self.memory[self.instruction_pointer + 2],
            &instruction_modes[1],
        );
        let result = left_operand * right_operand;
        self.memory.write_value(
            self.memory[self.instruction_pointer + 3],
            result,
            &instruction_modes[2],
        );
        StepResult::NextInstruction
    }

    fn run_instruction_input(&mut self, instruction_modes: &[InstructionMode]) -> StepResult {
        let next_input = self.input_queue.pop_front();
        if let Some(input) = next_input {
            self.memory.write_value(
                self.memory[self.instruction_pointer + 1],
                input,
                &instruction_modes[0],
            );
            return StepResult::NextInstruction;
        }
        StepResult::ReadInput
    }

    fn run_instruction_output(&mut self, instruction_modes: &[InstructionMode]) -> StepResult {
        let operand = self.memory.read_value(
            self.memory[self.instruction_pointer + 1],
            &instruction_modes[0],
        );
        StepResult::ProduceOutput(operand)
    }

    fn run_instruction_jump_if_true(
        &mut self,
        instruction_modes: &[InstructionMode],
    ) -> StepResult {
        let operand = self.memory.read_value(
            self.memory[self.instruction_pointer + 1],
            &instruction_modes[0],
        );
        if operand != 0 {
            let jump_address: Address = self
                .memory
                .read_value(
                    self.memory[self.instruction_pointer + 2],
                    &instruction_modes[1],
                )
                .try_into()
                .unwrap();
            return StepResult::SetInstructionPointerTo(jump_address);
        }
        StepResult::NextInstruction
    }

    // TODO remove duplicated code
    fn run_instruction_jump_if_false(
        &mut self,
        instruction_modes: &[InstructionMode],
    ) -> StepResult {
        let operand = self.memory.read_value(
            self.memory[self.instruction_pointer + 1],
            &instruction_modes[0],
        );
        if operand == 0 {
            let jump_address: Address = self
                .memory
                .read_value(
                    self.memory[self.instruction_pointer + 2],
                    &instruction_modes[1],
                )
                .try_into()
                .unwrap();
            self.instruction_pointer = jump_address;
            return StepResult::SetInstructionPointerTo(jump_address);
        }
        StepResult::NextInstruction
    }

    fn run_instruction_less_than(&mut self, instruction_modes: &[InstructionMode]) -> StepResult {
        let first_parameter = self.memory.read_value(
            self.memory[self.instruction_pointer + 1],
            &instruction_modes[0],
        );
        let second_parameter = self.memory.read_value(
            self.memory[self.instruction_pointer + 2],
            &instruction_modes[1],
        );
        let to_store = if first_parameter < second_parameter {
            1
        } else {
            0
        };
        self.memory.write_value(
            self.memory[self.instruction_pointer + 3],
            to_store,
            &instruction_modes[2],
        );
        StepResult::NextInstruction
    }

    // TODO remove duplicated code
    fn run_instruction_equals(&mut self, instruction_modes: &[InstructionMode]) -> StepResult {
        let first_parameter = self.memory.read_value(
            self.memory[self.instruction_pointer + 1],
            &instruction_modes[0],
        );
        let second_parameter = self.memory.read_value(
            self.memory[self.instruction_pointer + 2],
            &instruction_modes[1],
        );
        let to_store = if first_parameter == second_parameter {
            1
        } else {
            0
        };
        self.memory.write_value(
            self.memory[self.instruction_pointer + 3],
            to_store,
            &instruction_modes[2],
        );
        StepResult::NextInstruction
    }

    #[allow(dead_code)]
    fn run_instruction_adjust_relative_base(
        &mut self,
        instruction_modes: &[InstructionMode],
    ) -> StepResult {
        let new_base = self.memory.read_value(
            self.memory[self.instruction_pointer + 1],
            &instruction_modes[0],
        );
        self.memory
            .adjust_relative_base(new_base.try_into().unwrap());
        StepResult::NextInstruction
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
    AdjustRelativeBase,
    Halt,
}

#[derive(PartialEq, Debug, Clone)]
enum InstructionMode {
    Position,
    Immediate,
    Relative,
}

#[derive(PartialEq, Debug)]
struct InstructionDefinition {
    opcode: Opcode,
    instruction_modes: Vec<InstructionMode>,
}

fn parse_instruction_definition(instruction_code: Value) -> Option<InstructionDefinition> {
    if instruction_code < 0 {
        panic!("Can not parse opcode: {}", instruction_code);
    }
    let opcode = parse_opcode(instruction_code % 100)
        .unwrap_or_else(|| panic!("Can not parse opcode: {}", instruction_code));
    // TODO some validation? make sure length matches opcode?
    let instruction_modes = parse_instruction_modes(instruction_code / 100)
        .unwrap_or_else(|| panic!("Can not parse instruction modes: {}", instruction_code));
    Some(InstructionDefinition {
        opcode,
        instruction_modes,
    })
}

// TODO: "Instruction" trait, with "static length" + "run" function
// would allow to keep length and run close to each other
fn instruction_length(opcode: Opcode) -> usize {
    match opcode {
        Opcode::Addition => 4,
        Opcode::Multiplication => 4,
        Opcode::Input => 2,
        Opcode::Output => 2,
        Opcode::JumpIfTrue => 3,
        Opcode::JumpIfFalse => 3,
        Opcode::LessThan => 4,
        Opcode::Equals => 4,
        Opcode::AdjustRelativeBase => 2,
        Opcode::Halt => 0,
    }
}

fn parse_opcode(opcode_int: Value) -> Option<Opcode> {
    match opcode_int {
        1 => Some(Opcode::Addition),
        2 => Some(Opcode::Multiplication),
        3 => Some(Opcode::Input),
        4 => Some(Opcode::Output),
        5 => Some(Opcode::JumpIfTrue),
        6 => Some(Opcode::JumpIfFalse),
        7 => Some(Opcode::LessThan),
        8 => Some(Opcode::Equals),
        9 => Some(Opcode::AdjustRelativeBase),
        99 => Some(Opcode::Halt),
        _ => None,
    }
}

fn parse_instruction_modes(instruction_modes_int: Value) -> Option<Vec<InstructionMode>> {
    let mut instruction_modes = Vec::new();
    let mut instruction_modes_int = instruction_modes_int;
    loop {
        instruction_modes.push(match instruction_modes_int % 10 {
            0 => InstructionMode::Position,
            1 => InstructionMode::Immediate,
            2 => InstructionMode::Relative,
            _ => return None,
        });
        if instruction_modes_int < 10 {
            // always return a vector of size at least 2 to simplify
            if instruction_modes.len() == 1 {
                instruction_modes.push(InstructionMode::Position);
            }
            if instruction_modes.len() == 2 {
                instruction_modes.push(InstructionMode::Position);
            }
            return Some(instruction_modes);
        }
        instruction_modes_int /= 10;
    }
}

// TODO more tests, make a distinction between unit and integration tests
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
    fn test_interpreter_run_quine() {
        let quine = "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99";
        let mut interpreter = Interpreter::from_code(&quine).unwrap();
        let mut outputs: Vec<String> = Vec::new();
        loop {
            let output = interpreter.run_until_block().unwrap();
            match output {
                IOAction::Halt => {
                    assert_eq!(outputs, quine.split(",").collect::<Vec<&str>>());
                    return;
                }
                IOAction::ProduceOutput(x) => {
                    outputs.push(x.to_string());
                }
                _ => assert!(false, "unexpected output type"),
            }
        }
    }

    #[test]
    fn test_interpreter_output() {
        assert_interpreter_single_output("1102,34915192,34915192,7,4,7,99,0", 1219070632396864);
        assert_interpreter_single_output("104,1125899906842624,99", 1125899906842624);
        assert_interpreter_single_output("1102,6,7,1985,109,2000,109,19,204,-34,99", 42);
    }

    #[test]
    fn test_interpreter_run_immediate_mode() {
        assert_interpreter_run("1002,4,3,4,33", "1002,4,3,4,99");
    }

    #[test]
    fn test_interpreter_overflow() {
        assert_interpreter_single_output("109,10,21102,6,7,-5,4,5,99", 42);
    }

    fn assert_interpreter_run(starting_memory: &str, expected_memory: &str) {
        let mut interpreter = Interpreter::from_code(starting_memory).unwrap();
        let output = interpreter.run_until_block();
        assert!(output.is_ok());
        let expected_memory = Memory::from_string(expected_memory).unwrap();
        assert_eq!(interpreter.memory, expected_memory);
    }

    fn assert_interpreter_single_output(starting_memory: &str, expected_output: isize) {
        let mut interpreter = Interpreter::from_code(starting_memory).unwrap();
        let output = interpreter.run_until_block();
        assert!(output.is_ok());
        match output.unwrap() {
            IOAction::ProduceOutput(value) => assert_eq!(value, expected_output),
            _ => assert!(false, "unexpected output type"),
        }
        assert_eq!(interpreter.run_until_block().unwrap(), IOAction::Halt);
    }

    #[test]
    fn test_parse_instruction_definition() {
        assert_eq!(
            Some(InstructionDefinition {
                opcode: Opcode::Multiplication,
                instruction_modes: Vec::from(
                    [
                        InstructionMode::Position,
                        InstructionMode::Immediate,
                        InstructionMode::Position
                    ]
                    .to_vec()
                )
            }),
            parse_instruction_definition(1002)
        );

        assert_eq!(
            Some(InstructionDefinition {
                opcode: Opcode::Input,
                instruction_modes: Vec::from(
                    [
                        InstructionMode::Position,
                        InstructionMode::Position,
                        InstructionMode::Position
                    ]
                    .to_vec()
                )
            }),
            parse_instruction_definition(3)
        );
    }
}
