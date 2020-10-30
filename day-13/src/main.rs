use std::collections::HashMap;

mod intcode;

// part 1 only for now, quick and dirty
fn main() {
    let mut source_interpreter = intcode::Interpreter::from_file("input.txt").unwrap();
    let mut screen: HashMap<(isize, isize), isize> = HashMap::new();
    loop {
        match source_interpreter.run_until_block().unwrap() {
            intcode::IOAction::Halt => break,
            intcode::IOAction::ReadInput => {
                panic!("unexpected input instruction");
            }
            intcode::IOAction::ProduceOutput(output) => {
                let x = output;
                if let intcode::IOAction::ProduceOutput(y) =
                    source_interpreter.run_until_block().unwrap()
                {
                    if let intcode::IOAction::ProduceOutput(t) =
                        source_interpreter.run_until_block().unwrap()
                    {
                        screen.insert((x, y), t);
                    }
                }
            }
        }
    }
    let result = screen.values().filter(|v| **v == 2).count();
    println!("{}", result);
}
