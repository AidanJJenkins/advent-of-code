use std::io::BufRead;

use crate::days::one::open_sesame;

pub fn ro() -> Result<(), Box<dyn std::error::Error>> {
    let reader = open_sesame("day_2_input");

    let mut res1 = 0;
    let mut res2 = 0;

    for line in reader?.lines() {
        let line = line.unwrap();
        res1 += sham(&line);
        res2 += bo(&line);
    }

    println!("day 2a: {}", res1);
    println!("day 2b: {}", res2);
    Ok(())
}

fn sham(round: &str) -> usize {
    let mut score = 0;

    match round {
        "A X" => score += 4,
        "A Y" => score += 8,
        "A Z" => score += 3,
        "B X" => score += 1,
        "B Y" => score += 5,
        "B Z" => score += 9,
        "C X" => score += 7,
        "C Y" => score += 2,
        "C Z" => score += 6,
        _ => {}
    }

    score
}

fn bo(round: &str) -> usize {
    let mut score = 0;

    match round {
        "A X" => score += 3,
        "A Y" => score += 4,
        "A Z" => score += 8,
        "B X" => score += 1,
        "B Y" => score += 5,
        "B Z" => score += 9,
        "C X" => score += 2,
        "C Y" => score += 6,
        "C Z" => score += 7,
        _ => {}
    }
    score
}
