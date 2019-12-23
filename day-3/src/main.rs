// this time, let's see what a concise, less readable solution would look like
// no error handling whatsoever!

use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::{HashSet, HashMap};
use std::iter::FromIterator;

fn main() {
    part_1();
    part_2();
}

fn part_1() {
    let inputs: Vec<String> = BufReader::new(File::open("input.txt").unwrap()).lines().map(|l| l.unwrap()).collect();
    let first_points = make_set_of_points(&inputs[0]);
    let second_points = make_set_of_points(&inputs[1]);
    let intersections = first_points.intersection(&second_points);
    let mut sorted = intersections.copied().collect::<Vec<(isize, isize)>>();
    sorted.sort_unstable_by(|a, b| (a.0.abs() + a.1.abs()).cmp(&(b.0.abs() + b.1.abs())));
    let result = sorted[0];
    println!("{}", result.0.abs() + result.1.abs());
}

fn part_2() {
    let inputs: Vec<String> = BufReader::new(File::open("input.txt").unwrap()).lines().map(|l| l.unwrap()).collect();
    let first_points_with_distance = make_set_of_points_with_length(&inputs[0]);
    let second_points_with_distance = make_set_of_points_with_length(&inputs[1]);

    let first_points: HashSet<&(isize, isize)> = HashSet::from_iter(first_points_with_distance.keys());
    let second_points: HashSet<&(isize, isize)> = HashSet::from_iter(second_points_with_distance.keys());
    let intersections = first_points.intersection(&second_points);

    let mut min_distance_found_so_far = std::isize::MAX;
    for intersection in intersections {
        let distance = first_points_with_distance.get(intersection).unwrap() + second_points_with_distance.get(intersection).unwrap();
        min_distance_found_so_far = std::cmp::min(distance, min_distance_found_so_far);
    }

    println!("{}", min_distance_found_so_far);
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

fn make_set_of_points_with_length(input: &str) -> HashMap<(isize, isize), isize> {
    let lines = input.split(',');
    let mut result = HashMap::new();
    let mut location: (isize, isize) = (0, 0);
    let mut distance = 0;
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
            distance += 1;
            result.insert(location, distance);
            length -= 1;
        }
    }
    result
}
