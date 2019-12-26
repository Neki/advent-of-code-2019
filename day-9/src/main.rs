mod intcode;

fn main() {
    part_1();
    part_2();
}

fn part_1() {
    let mut source_interpreter = intcode::Interpreter::from_file("input.txt").unwrap();
    source_interpreter.queue_input(1);
    source_interpreter.run_interactively().unwrap();
}

fn part_2() {
    let mut source_interpreter = intcode::Interpreter::from_file("input.txt").unwrap();
    source_interpreter.queue_input(2);
    source_interpreter.run_interactively().unwrap();
}
