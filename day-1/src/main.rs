use std::error;
use std::cmp;
use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() -> Result<(), Box<dyn error::Error>> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut total_fuel = 0;
    for line in reader.lines() {
        let module_mass = line?.parse::<i32>()?;
        // let's ignore overflows
        // change this to just `compute_fuel_for_mass` for part 1
        total_fuel += compute_total_fuel_for_mass(module_mass);
    }
    println!("{}", total_fuel);

    Ok(())
}

// there's probably a nice formula for that, but why do the machine job?
fn compute_total_fuel_for_mass(mass: i32) -> i32 {
    if mass == 0 {
        return 0
    }
    let fuel = compute_fuel_for_mass(mass);
    fuel + compute_total_fuel_for_mass(fuel)
}

fn compute_fuel_for_mass(mass: i32) -> i32 {
    cmp::max(mass / 3 - 2, 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_fuel_for_mass() {
        assert_eq!(compute_fuel_for_mass(12), 2);
        assert_eq!(compute_fuel_for_mass(14), 2);
        assert_eq!(compute_fuel_for_mass(1969), 654);
        assert_eq!(compute_fuel_for_mass(100756), 33583);
        assert_eq!(compute_fuel_for_mass(0), 0);
        assert_eq!(compute_fuel_for_mass(1), 0);
        assert_eq!(compute_fuel_for_mass(2), 0);
        assert_eq!(compute_fuel_for_mass(3), 0);
    }

    #[test]
    fn test_compute_total_fuel_for_mass() {
        assert_eq!(compute_total_fuel_for_mass(14), 2);
        assert_eq!(compute_total_fuel_for_mass(1969), 966);
        assert_eq!(compute_total_fuel_for_mass(100756), 50346);
    }

}
