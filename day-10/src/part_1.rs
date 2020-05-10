use std::collections::{BTreeMap, BTreeSet};
use std::error;
use std::fs::File;
use std::io::Read;

fn part_1() {
    let map = load_from_file("input.txt").unwrap();
    let visibility = compute_visibility_map(&map);
    let result = visibility.values().max().unwrap();
    println!("{}", result);
}

fn compute_visibility_map(map: &AsteroidMap) -> BTreeMap<(isize, isize), isize> {
    let mut visibility_map: BTreeMap<(isize, isize), isize> = BTreeMap::new();
    for (x, y) in map.asteroids.iter() {
        let mut visibility_count = 0;
        for (other_x, other_y) in map.asteroids.iter() {
            if is_visible_from((*other_x, *other_y), (*x, *y), &map) {
                visibility_count += 1;
            }
        }
        visibility_map.insert((*x, *y), visibility_count);
    }
    visibility_map
}

fn is_visible_from(asteroid: (isize, isize), observer: (isize, isize), map: &AsteroidMap) -> bool {
    if asteroid == observer {
        return false;
    }
    let direction_x = asteroid.0 - observer.0;
    let direction_y = asteroid.1 - observer.1;
    let direction = Direction::new(direction_x, direction_y);
    let mut next_point = direction.add_to_coordinates(observer.0, observer.1);
    while next_point != asteroid {
        // if there's an asteroid on the path, return false
        if map.asteroids.contains(&(next_point.0, next_point.1)) {
            return false;
        }
        next_point = direction.add_to_coordinates(next_point.0, next_point.1);
    }
    true
}

struct Direction {
    x: isize,
    y: isize,
}

impl Direction {
    fn new(x: isize, y: isize) -> Self {
        let divisor = gcd(x, y).abs();
        Self {
            x: x / divisor,
            y: y / divisor,
        }
    }

    fn add_to_coordinates(&self, x: isize, y: isize) -> (isize, isize) {
        (self.x + x, self.y + y)
    }
}

struct AsteroidMap {
    asteroids: BTreeSet<(isize, isize)>,
}

fn gcd(a: isize, b: isize) -> isize {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

fn load_from_file(path: &str) -> Result<AsteroidMap, Box<dyn error::Error>> {
    let mut file = File::open(path)?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;
    let mut x = 0;
    let mut y = 0;
    let mut asteroids = BTreeSet::new();
    for c in buffer.chars() {
        x += 1;
        if c == '\n' {
            y += 1;
            x = 0;
        }
        if c == '#' {
            asteroids.insert((x, y));
        }
    }
    Ok(AsteroidMap { asteroids })
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(10, 1), 1);
        assert_eq!(gcd(6, 4), 2);
        assert_eq!(gcd(-6, 4), -2);
    }
}
