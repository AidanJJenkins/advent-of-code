use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn open_sesame(file: &str) -> Result<BufReader<File>, Box<dyn std::error::Error>> {
    let input = format!(
        "/Users/fognozzle/work/src/github.com/aidanjjenkins/advent_of_code/inputs/{}.txt",
        file
    );
    let file = File::open(input)?;
    let reader = BufReader::new(file);
    Ok(reader)
}

pub fn cals() -> Result<(), Box<dyn std::error::Error>> {
    let reader = open_sesame("day_1_input");

    let mut r = 0;
    let mut t = 0;
    for i in reader?.lines() {
        let line = i?;

        if line.trim().is_empty() {
            if t > r {
                r = t;
            }
            t = 0;
        } else {
            if let Ok(num) = line.trim().parse::<u32>() {
                t += num;
            } else {
                println!("Error parsing value: {}", line);
            }
        }
    }
    println!("day 1: {}", r);
    Ok(())
}
