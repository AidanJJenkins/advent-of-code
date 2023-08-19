use rust::days::{one::cals, three::{ruck, ruck_part_2}, two::ro};

fn main() {
    if let Err(error) = cals() {
        eprintln!("Error: {}", error);
    }
    if let Err(error) = ro() {
        eprintln!("Error: {}", error);
    }
    if let Err(error) = ruck() {
        eprintln!("Error: {}", error);
    }
    if let Err(error) = ruck_part_2() {
        eprintln!("Error: {}", error);
    }
}
