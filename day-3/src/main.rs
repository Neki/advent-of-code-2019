// this time, let's see what a concise, less readable solution would look like
// no error handling whatsoever!

use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashSet;

fn main() {
    let inputs: Vec<String> = BufReader::new(File::open("input.txt").unwrap()).lines().map(|l| l.unwrap()).collect();
    let first_points = make_set_of_points(&inputs[0]);
    let second_points = make_set_of_points(&inputs[1]);
    let intersections = first_points.intersection(&second_points);
    let mut sorted: Vec<(isize, isize)> = intersections.copied().collect::<Vec<(isize, isize)>>();
    sorted.sort_unstable_by(|a, b| (a.0.abs() + a.1.abs()).cmp(&(b.0.abs() + b.1.abs())));
    let result = sorted[0];
    println!("{}", result.0.abs() + result.1.abs());
}

fn make_set_of_points(input: &str) -> HashSet<(isize, isize)> {
    let lines = input.split(',');
    let mut result = HashSet::new();
    let mut location: (isize, isize) = (0, 0);
    for line in lines {
        let direction = line.chars().nth(0).unwrap();
        let mut length = *&line[1..].parse::<isize>().unwrap();
        while length > 0 {
            match direction {
                'U' => location.0 += 1,
                'D' => location.0 -= 1,
                'R' => location.1 += 1,
                'L' => location.1 -= 1,
                _ => panic!("unknown direction")
            }
            result.insert(location);
            length -= 1;
        }
    }
    result
}
