use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::error;
use std::fs::File;
use std::io::Read;

fn main() {
    // load data
    let map = load_from_file("input.txt").unwrap();
    // part 1
    let (station_location, number_of_visible_asteroids) = get_best_station_location(&map);
    println!("{}", number_of_visible_asteroids);
    println!(
        "best_x:{} best_y:{}",
        station_location.x, station_location.y
    );

    // part 2
    let mut asteroid_list = map.iter().collect::<Vec<&Point>>();

    // sort asteroids according the order they will be destroyed by the laser
    // the order is defined by:
    // first, the number of other asteroids between the asteroid and the laser source
    // for two asteroids with the same "distance" (defined in number of asteroids),
    // we compare their position clockwise
    asteroid_list
        .sort_by_key(|asteroid| asteroid.get_relative_coordinates_from(station_location, &map));

    // first asteroid in the list is our laser station, so there's no off-by-one error here
    let best_asteroid = asteroid_list.get(200).unwrap();
    println!(
        "asteroid 200: ({}, {}) -> {}",
        best_asteroid.x,
        best_asteroid.y,
        best_asteroid.x * 100 + best_asteroid.y
    );
}

fn load_from_file(path: &str) -> Result<AsteroidMap, Box<dyn error::Error>> {
    let mut file = File::open(path)?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;
    let mut x = 0;
    let mut y = 0;
    let mut asteroids = BTreeSet::new();
    for c in buffer.chars() {
        if c == '#' {
            asteroids.insert(Point { x, y });
        }
        x += 1;
        if c == '\n' {
            y += 1;
            x = 0;
        }
    }
    Ok(asteroids)
}

fn get_best_station_location(map: &AsteroidMap) -> (&Point, isize) {
    // not sure if this is the best way - I am trying to avoid calling
    // compute_number_of_visible_asteroids_from again on the best asteroid
    map.iter()
        .map(|asteroid| {
            (
                asteroid,
                compute_number_of_visible_asteroids_from(asteroid, &map),
            )
        })
        .max_by_key(|element| element.1)
        // assume map is not empty
        .unwrap()
}

fn compute_number_of_visible_asteroids_from(asteroid: &Point, map: &AsteroidMap) -> isize {
    let mut visible_asteroids = 0;
    for other_asteroid in map {
        if asteroid == other_asteroid {
            continue;
        }
        let is_visible = get_number_of_asteroids_between(asteroid, other_asteroid, map) == 0;
        if is_visible {
            visible_asteroids += 1;
        }
    }
    visible_asteroids
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Point {
    x: isize,
    y: isize,
}

// used to sort asteroids (see comments in main function)
#[derive(PartialEq, PartialOrd, Copy, Clone)]
struct RelativeCoordinates {
    // number of asteroids between the laser source and the current asteroid
    nb_asteroids_distance: isize,
    // a number that is higher for asteroids that are farther clockwise (origin at the laser
    // source), see schema and compute method in get_relative_coordinates_from
    angle_comparator: f64,
}

// hack incoming - ignore NaN floats
impl Eq for RelativeCoordinates {}
impl Ord for RelativeCoordinates {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}

type AsteroidMap = BTreeSet<Point>;

struct DirectionVector {
    xdelta: isize,
    ydelta: isize,
}

fn gcd(a: isize, b: isize) -> isize {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

impl DirectionVector {
    fn new(xdelta: isize, ydelta: isize) -> Self {
        let divisor = gcd(xdelta, ydelta).abs();
        Self {
            xdelta: xdelta / divisor,
            ydelta: ydelta / divisor,
        }
    }

    fn add_to_coordinates(&self, point: &Point) -> Point {
        Point {
            x: self.xdelta + point.x,
            y: self.ydelta + point.y,
        }
    }
}

impl Point {
    fn get_relative_coordinates_from(
        &self,
        asteroid: &Point,
        map: &AsteroidMap,
    ) -> RelativeCoordinates {
        let distance = get_number_of_asteroids_between(&self, asteroid, map);
        //  -> x
        // |  . . .dx . .
        // v  . . +-- # .
        // y  . dy| / . .
        //    . . * . . .
        //    . . . . . .
        //
        // * = laser source / station location (= asteroid parameter)
        // # = asteroid under examination (= self parameter)
        // we're looking for the angle beween *+ and *# (clockwise)
        //
        let dx = (self.x - asteroid.x) as f64;
        let dy = (self.y - asteroid.y) as f64;
        // there's a solution using discrete math only,
        // (sin, cos and gcd could help) but I'm not motivated enough
        // to write it
        let angle = if dy == 0.0 && dx == 0.0 {
            // ensure (0, 0) finished in first position after the sort
            -10.0
        } else {
            -dx.atan2(dy)
        };
        RelativeCoordinates {
            nb_asteroids_distance: distance,
            angle_comparator: angle,
        }
    }
}

fn get_number_of_asteroids_between(
    asteroid_1: &Point,
    asteroid_2: &Point,
    map: &AsteroidMap,
) -> isize {
    if asteroid_1 == asteroid_2 {
        return 0;
    }
    let direction = DirectionVector::new(asteroid_1.x - asteroid_2.x, asteroid_1.y - asteroid_2.y);
    let mut location_to_examine: Point = *asteroid_2;
    let mut count = 0;
    while location_to_examine != *asteroid_1 {
        location_to_examine = direction.add_to_coordinates(&location_to_examine);
        if map.contains(&location_to_examine) {
            count += 1;
        }
    }
    // remove asteroid_1 from the count
    count - 1
}
