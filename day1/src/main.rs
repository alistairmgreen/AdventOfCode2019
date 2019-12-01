use std::error::Error;
use std::fs::File;
use std::io;
use std::io::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("puzzle_input.txt")?;
    let reader = io::BufReader::new(file);
    
    let mut total_fuel = 0.0;

    for line in reader.lines() {
        let module_mass: f64 = line?.parse()?;
        total_fuel += fuel(module_mass);
    }

    println!("Total fuel required = {:0.0}", total_fuel);
    
    Ok(())
}

fn fuel(mass: f64) -> f64 {
    f64::floor(mass / 3.0) - 2.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fuel_example_1() {
        assert_eq!(fuel(12.0) as i32, 2);
    }

    #[test]
    fn fuel_example_2() {
        assert_eq!(fuel(14.0) as i32, 2);
    }

    #[test]
    fn fuel_example_3() {
        assert_eq!(fuel(1969.0) as i32, 654);
    }

    #[test]
    fn fuel_example_4() {
        assert_eq!(fuel(100_756.0) as i32, 33583);
    }
}