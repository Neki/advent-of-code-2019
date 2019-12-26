use std::io::Read;
use std::fs::File;

fn main() {
    part_1();
    part_2();
}


fn part_1() {
    let filename = "input.txt";
    let mut file = File::open(filename).unwrap();
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();

    let mut min_number_of_0s = std::usize::MAX;
    let mut checksum = 0;
    let mut number_of_1s = 0;
    let mut number_of_2s = 0;
    let mut number_of_0s = 0;

    let mut layer_position = 0;

    const LAYER_SIZE: usize = 25 * 6;


    for digit_str in buffer.trim().chars() {
        // could by simplified by using group_by from the itertools crate?
        let parsed_digit = digit_str.to_digit(10).unwrap();
        match parsed_digit {
            1 => { number_of_1s += 1; }
            2 => { number_of_2s += 1; }
            0 => { number_of_0s += 1; }
            _ => { panic!("woops") }
        }
        layer_position += 1;
        if layer_position == LAYER_SIZE {
            layer_position = 0;
            if min_number_of_0s > number_of_0s {
                checksum = number_of_1s * number_of_2s;
                min_number_of_0s = number_of_0s;
            }
            number_of_1s = 0;
            number_of_2s = 0;
            number_of_0s = 0;
        }
    }

    println!("{}", checksum);
}

fn part_2() {
    let filename = "input.txt";
    let mut file = File::open(filename).unwrap();
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();

    let mut layer_position = 0;

    const LAYER_SIZE: usize = 25 * 6;
    let mut colors = [2; LAYER_SIZE];

    for digit_str in buffer.trim().chars() {
        // could by simplified by using group_by from the itertools crate?
        let parsed_digit = digit_str.to_digit(10).unwrap();

        if colors[layer_position] == 2 {
            colors[layer_position] = parsed_digit;
        }

        layer_position += 1;

        if layer_position == LAYER_SIZE {
            layer_position = 0;
        }
    }

    for i in 0..6 {
        for digit in &colors[(i* 25)..((i+1)*25)] {
            if digit == &1 {
                print!("{}", digit);
            } else {
                print!(" ");
            }
        }
        println!("");
    }

}
