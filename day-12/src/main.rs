// use std::fs::read_to_string;
use std::ops::Add;
use std::thread;

#[derive(Clone, Copy, PartialEq, Eq)]
struct Position {
    x: isize,
    y: isize,
    z: isize,
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Velocity {
    x: isize,
    y: isize,
    z: isize,
}

impl Velocity {
    fn new(x: isize, y: isize, z: isize) -> Self {
        Velocity { x, y, z }
    }

    fn kinetic_energy(&self) -> isize {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

impl Position {
    fn add_velocity(&self, velocity: &Velocity) -> Self {
        Position {
            x: self.x + velocity.x,
            y: self.y + velocity.y,
            z: self.z + velocity.z,
        }
    }

    fn new(x: isize, y: isize, z: isize) -> Self {
        Position { x, y, z }
    }

    fn potential_energy(&self) -> isize {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

impl Add for Velocity {
    type Output = Velocity;

    fn add(self, other: Velocity) -> Velocity {
        Velocity {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Body {
    position: Position,
    velocity: Velocity,
}

impl Body {
    fn apply_gravity(&self, position: &Position) -> Velocity {
        let velocity_diff = Velocity::new(
            (position.x - self.position.x).signum(),
            (position.y - self.position.y).signum(),
            (position.z - self.position.z).signum(),
        );
        self.velocity + velocity_diff
    }

    fn apply_gravity_to_own_velocity(&mut self, position: &Position) {
        self.velocity = self.apply_gravity(position);
    }

    fn apply_velocity_to_own_position(&mut self) {
        self.position = self.position.add_velocity(&self.velocity);
    }

    fn new(position: &Position, velocity: &Velocity) -> Self {
        Body {
            position: *position,
            velocity: *velocity,
        }
    }

    fn energy(&self) -> isize {
        self.position.potential_energy() * self.velocity.kinetic_energy()
    }
}

fn run_simulation_step(bodies: &mut Vec<Body>) {
    // https://docs.rs/itertools/0.8.0/itertools/trait.Itertools.html#method.combinations
    // could come in handy, but let's do it manually
    for i in 0..(bodies.len() - 1) {
        for j in (i + 1)..bodies.len() {
            let (head, tail) = bodies.split_at_mut(i + 1);
            let body_1 = head.get_mut(i).unwrap();
            let body_2 = tail.get_mut(j - i - 1).unwrap();
            body_1.apply_gravity_to_own_velocity(&body_2.position);
            body_2.apply_gravity_to_own_velocity(&body_1.position);
        }
    }

    for i in 0..bodies.len() {
        bodies.get_mut(i).unwrap().apply_velocity_to_own_position();
    }
}

fn get_hardcoded_input() -> Vec<Body> {
    // let input = read_to_string("input.txt").expect("could not read input");
    // save the parsing, hardcode what we want! (we could use a regexp to parse this)
    // <x=-2, y=9, z=-5>
    // <x=16, y=19, z=9>
    // <x=0, y=3, z=6>
    // <x=11, y=0, z=11>
    let mut bodies = Vec::new();
    bodies.push(Body::new(
        &Position::new(-2, 9, -5),
        &Velocity::new(0, 0, 0),
    ));
    bodies.push(Body::new(
        &Position::new(16, 19, 9),
        &Velocity::new(0, 0, 0),
    ));
    bodies.push(Body::new(&Position::new(0, 3, 6), &Velocity::new(0, 0, 0)));
    bodies.push(Body::new(
        &Position::new(11, 0, 11),
        &Velocity::new(0, 0, 0),
    ));
    bodies
}

fn part_1() {
    let mut bodies = get_hardcoded_input();
    let steps = 1000;

    for _n in 0..steps {
        run_simulation_step(&mut bodies);
    }

    let mut total_energy = 0;
    for body in bodies {
        total_energy += body.energy();
    }
    println!("{}", total_energy);
}

fn main() {
    part_1();
    part_2_optimized_but_ugly();
}

fn part_2_optimized_but_ugly() {
    // this simulation is independant accross x, y and z axis
    // so we can parallelize these axis on separate threads, and have each child thread exits
    // when it got back into the initial conditions.
    // then, compute the lowest common multiples between these number of steps.
    // (note: we could build a clean solution upon the part 1 code, without even spawning threads, but...)
    let initial_xs = [-2, 16, 0, 11];
    let initial_ys = [9, 19, 3, 0];
    let initial_zs = [-5, 9, 6, 11];

    let mut children = Vec::new();
    let child_x = thread::spawn(move || run_simulation_1d(initial_xs));
    let child_y = thread::spawn(move || run_simulation_1d(initial_ys));
    let child_z = thread::spawn(move || run_simulation_1d(initial_zs));
    children.push(child_x);
    children.push(child_y);
    children.push(child_z);
    let results: Vec<usize> = children.into_iter().map(|t| t.join().unwrap()).collect();
    let lcm_step = results.into_iter().fold(1, |acc, x| lcm(x, acc));
    println!("{}", lcm_step);
}

fn run_simulation_1d(initial_xs: [isize; 4]) -> usize {
    let mut xs = initial_xs;
    let mut dxs = [0; 4];
    let mut steps: usize = 0;
    loop {
        steps += 1;
        // update velocities
        // looks like a candidate for SIMD instructions, if the compiler does not apply them by
        // itself
        dxs[0] -= (xs[0] - xs[1]).signum() + (xs[0] - xs[2]).signum() + (xs[0] - xs[3]).signum();
        dxs[1] -= (xs[1] - xs[0]).signum() + (xs[1] - xs[2]).signum() + (xs[1] - xs[3]).signum();
        dxs[2] -= (xs[2] - xs[0]).signum() + (xs[2] - xs[1]).signum() + (xs[2] - xs[3]).signum();
        dxs[3] -= (xs[3] - xs[0]).signum() + (xs[3] - xs[1]).signum() + (xs[3] - xs[2]).signum();
        // update positions
        xs[0] += dxs[0];
        xs[1] += dxs[1];
        xs[2] += dxs[2];
        xs[3] += dxs[3];
        // did we get in the initial state again?
        if xs == initial_xs && dxs == [0, 0, 0, 0] {
            return steps;
        }
    }
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

fn gcd(a: usize, b: usize) -> usize {
    let mut max = a;
    let mut min = b;
    if min > max {
        std::mem::swap(&mut min, &mut max);
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}
