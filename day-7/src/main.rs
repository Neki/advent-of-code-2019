// Is this cheating? Maybe, but since https://doc.rust-lang.org/1.1.0/std/slice/struct.Permutations.html is available in unstable...
use permutohedron::Heap;

mod intcode;

fn main() {
    part_1();
    part_2();
}

fn init_interpreters(
    source_interpreter: &intcode::Interpreter,
    inputs: &[isize],
) -> Vec<intcode::Interpreter> {
    let mut interpreters: Vec<intcode::Interpreter> = Vec::with_capacity(inputs.len());
    for input in inputs {
        let mut interpreter = source_interpreter.clone();
        interpreter.queue_input(*input);
        interpreters.push(interpreter);
    }
    interpreters
}

fn part_1() {
    let source_interpreter = intcode::Interpreter::from_file("input.txt").unwrap();
    let mut possible_inputs = vec![0, 1, 2, 3, 4];
    let heap = Heap::new(&mut possible_inputs);
    let mut max_output = std::isize::MIN;
    for inputs in heap {
        let mut interpreters = init_interpreters(&source_interpreter, &inputs);
        interpreters[0].queue_input(0);
        let mut next_input = 0;
        for i in 0..5 {
            let interpreter_output = interpreters[i].run_until_block();
            match interpreter_output {
                Ok(intcode::IOAction::ProduceOutput(x)) => next_input = x,
                _ => panic!(
                    "Interpreter should have given an output, but got {:?}",
                    interpreter_output
                ),
            }
            if i < 4 {
                interpreters[i + 1].queue_input(next_input);
            }
        }
        max_output = std::cmp::max(next_input, max_output);
    }
    println!("{}", max_output);
}

fn part_2() {
    let source_interpreter = intcode::Interpreter::from_file("input.txt").unwrap();
    let mut possible_inputs = vec![5, 6, 7, 8, 9];
    let heap = Heap::new(&mut possible_inputs);
    let mut max_output = std::isize::MIN;
    for inputs in heap {
        let mut interpreters = init_interpreters(&source_interpreter, &inputs);
        let mut next_input: isize = 0;
        let mut current_interpreter_index = 0;
        // The "cycle" iterator method require Self: Clone,
        // so I don't see how to use this iterator here (I don't want to
        // clone the interpreters, just reset the iterator)
        loop {
            let interpreter = &mut interpreters[current_interpreter_index];
            interpreter.queue_input(next_input);
            let interpreter_output = interpreter.run_until_block();
            match interpreter_output {
                Ok(intcode::IOAction::ProduceOutput(x)) => next_input = x,
                // Assumption: all interpreters will halt at the same time, so A is the first
                // interpreter to halt
                Ok(intcode::IOAction::Halt) => break,
                _ => panic!(
                    "Interpreter {} should have given an output or halted, but got {:?}",
                    current_interpreter_index, interpreter_output
                ),
            }
            current_interpreter_index += 1;
            if current_interpreter_index >= 5 {
                current_interpreter_index = 0
            }
        }
        max_output = std::cmp::max(next_input, max_output);
    }
    println!("{}", max_output);
}
