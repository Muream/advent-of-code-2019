use std::fs::File;
use std::io::{prelude::*, BufReader};

pub fn main() {
    let path = "./src/day1/input.txt";
    let modules_mass = get_modules_mass(path);


    let mut total_fuel = 0;
    for mass in &modules_mass {
        let fuel = compute_fuel_for_mass(mass);
        total_fuel += fuel;
    }
    println!("Answer 1: {}", total_fuel);

    let mut total_fuel = 0;
    for mass in &modules_mass {
        let fuel = compute_fuel_for_module(mass);
        total_fuel += fuel;
    }
    println!("Answer 2: {}", total_fuel);
}

fn get_modules_mass(path: &str) -> Vec<i32> {
    let mut masses = vec![];
    let file = File::open(path).expect("couldn't read file");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let mass: i32 = line.unwrap().parse().unwrap();
        masses.push(mass)
    }

    masses
}

fn compute_fuel_for_module(mass: &i32) -> i32 {
    let mut should_continue = true;
    let mut total_fuel = compute_fuel_for_mass(mass);
    let mut fuel = total_fuel.clone();

    while should_continue {
        fuel = compute_fuel_for_mass(&fuel);
        if fuel <= 0 {
            should_continue = false;
        } else {
            total_fuel += fuel
        }
    }

    total_fuel
}

fn compute_fuel_for_mass(mass: &i32) -> i32 {
    ((*mass as f64 / 3.0).floor() - 2.0) as i32
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_mass_12() {
        assert_eq!(compute_fuel_for_mass(12), 2);
    }
    #[test]
    fn test_mass_14() {
        assert_eq!(compute_fuel_for_mass(14), 2);
    }
    #[test]
    fn test_mass_1969() {
        assert_eq!(compute_fuel_for_mass(1969), 654);
    }
    #[test]
    fn test_mass_100756() {
        assert_eq!(compute_fuel_for_mass(100756), 33583);
    }

    #[test]
    fn test_module_12() {
        assert_eq!(compute_fuel_for_module(12), 2);
    }
    #[test]
    fn test_module_1969() {
        assert_eq!(compute_fuel_for_module(1969), 966);
    }
    #[test]
    fn test_module_100756() {
        assert_eq!(compute_fuel_for_module(100756), 50346);
    }
}