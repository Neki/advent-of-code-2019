// Disclaimer: super unclean code (copy pasted, no short methods, etc.)
// I stopped as soon as it worked.
mod intcode;

use std::collections::BTreeSet;
use std::isize;

fn main() {
    part_1();
    part_2();
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(PartialOrd, Ord, PartialEq, Eq, Clone, Copy)]
struct Point {
    x: isize,
    y: isize,
}

fn part_2() {
    let mut source_interpreter = intcode::Interpreter::from_file("input.txt").unwrap();
    let mut robot_coordinates = Point { x: 0, y: 0 };
    let mut robot_direction = Direction::Up;
    let mut white_panels: BTreeSet<Point> = BTreeSet::new();
    let mut current_color = 1; // white
    source_interpreter.queue_input(current_color);
    loop {
        match source_interpreter.run_until_block().unwrap() {
            intcode::IOAction::Halt => break,
            intcode::IOAction::ReadInput => {
                source_interpreter.queue_input(current_color);
            }
            intcode::IOAction::ProduceOutput(output) => {
                if output == 0 {
                    // paint black
                    white_panels.remove(&robot_coordinates);
                } else if output == 1 {
                    // paint white
                    white_panels.insert(robot_coordinates);
                }
                let move_instruction = source_interpreter.run_until_block().unwrap();
                match move_instruction {
                    intcode::IOAction::ProduceOutput(rotation_code) => {
                        if rotation_code == 0 {
                            robot_direction = turn_left(&robot_direction);
                        } else if rotation_code == 1 {
                            robot_direction = turn_right(&robot_direction);
                        }
                        robot_coordinates = move_robot(&robot_coordinates, &robot_direction);
                    }
                    _ => panic!("unexpected program behaviour"),
                }
                current_color = if white_panels.contains(&robot_coordinates) {
                    // white
                    1
                } else {
                    // black
                    0
                };
            }
        }
    }

    // display result
    let mut previous_line: isize = white_panels.iter().nth(0).unwrap().x;
    let mut previous_column = white_panels.iter().nth(0).unwrap().y;
    print!(" ");
    for point in white_panels.iter() {
        for _ in previous_line..point.x {
            println!();
        }
        previous_line = point.x;
        for _ in previous_column..(point.y - 1) {
            print!(" ");
        }
        print!("+");
        previous_column = point.y;
    }
}

fn part_1() {
    let mut source_interpreter = intcode::Interpreter::from_file("input.txt").unwrap();
    let mut robot_coordinates = Point { x: 0, y: 0 };
    let mut robot_direction = Direction::Up;
    let mut white_panels: BTreeSet<Point> = BTreeSet::new();
    let mut painted_panels: BTreeSet<Point> = BTreeSet::new();
    let mut current_color = 0; // black
    source_interpreter.queue_input(current_color);
    loop {
        match source_interpreter.run_until_block().unwrap() {
            intcode::IOAction::Halt => break,
            intcode::IOAction::ReadInput => {
                source_interpreter.queue_input(current_color);
            }
            intcode::IOAction::ProduceOutput(output) => {
                if output == 0 {
                    // paint black
                    white_panels.remove(&robot_coordinates);
                } else if output == 1 {
                    // paint white
                    white_panels.insert(robot_coordinates);
                }
                painted_panels.insert(robot_coordinates);
                let move_instruction = source_interpreter.run_until_block().unwrap();
                match move_instruction {
                    intcode::IOAction::ProduceOutput(rotation_code) => {
                        if rotation_code == 0 {
                            robot_direction = turn_left(&robot_direction);
                        } else if rotation_code == 1 {
                            robot_direction = turn_right(&robot_direction);
                        }
                        robot_coordinates = move_robot(&robot_coordinates, &robot_direction);
                    }
                    _ => panic!("unexpected program behaviour"),
                }
                current_color = if white_panels.contains(&robot_coordinates) {
                    // white
                    1
                } else {
                    // black
                    0
                };
            }
        }
    }
    println!("{}", painted_panels.len());
}

fn move_robot(robot_coordinates: &Point, robot_direction: &Direction) -> Point {
    let x = robot_coordinates.x;
    let y = robot_coordinates.y;
    match robot_direction {
        Direction::Up => Point { x: x - 1, y },
        Direction::Left => Point { x, y: y - 1 },
        Direction::Down => Point { x: x + 1, y },
        Direction::Right => Point { x, y: y + 1 },
    }
}

fn turn_left(direction: &Direction) -> Direction {
    match direction {
        Direction::Up => Direction::Left,
        Direction::Left => Direction::Down,
        Direction::Down => Direction::Right,
        Direction::Right => Direction::Up,
    }
}

fn turn_right(direction: &Direction) -> Direction {
    match direction {
        Direction::Up => Direction::Right,
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
    }
}
